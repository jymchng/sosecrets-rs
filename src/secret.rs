use core::{
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::{Add, Deref, Drop},
};

use crate::traits::{ExposeSecret, SecretIntoInner};
use typenum::{IsLessOrEqual, Sum, True, Unsigned, U0, U1};
use zeroize::Zeroize;

#[cfg(feature = "cloneable-secret")]
use crate::traits::CloneableSecret;

pub type AddU1<A> = <A as core::ops::Add<U1>>::Output;

pub struct Secret<
    T: Zeroize,
    MEC: Unsigned,
    EC: Add<U1> + IsLessOrEqual<MEC, Output = True> + Unsigned = U0,
>(ManuallyDrop<T>, PhantomData<(MEC, EC)>);

pub struct ExposedSecret<'brand, T>(T, PhantomData<fn(&'brand ()) -> &'brand ()>);

impl<T: Zeroize, MEC: Unsigned> Secret<T, MEC, U0>
where
    U0: IsLessOrEqual<MEC, Output = True>,
{
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self(ManuallyDrop::new(value), PhantomData)
    }
}

impl<
        'max,
        T: Zeroize,
        MEC: Unsigned,
        EC: Add<U1> + Unsigned + IsLessOrEqual<MEC, Output = True>,
    > ExposeSecret<'max, &'max T, MEC, EC> for Secret<T, MEC, EC>
{
    type Exposed<'brand> = ExposedSecret<'brand, &'brand T>
    where
        'max: 'brand;

    type Next = Secret<T, MEC, Sum<EC, U1>>
    where
        EC: Add<U1> + Unsigned + IsLessOrEqual<MEC, Output = True>,
        Sum<EC, U1>: Unsigned + IsLessOrEqual<MEC, Output = True> + Add<U1>,
        T: Zeroize;

    #[inline(always)]
    fn expose_secret<ReturnType, ClosureType>(
        mut self,
        scope: ClosureType,
    ) -> (Secret<T, MEC, AddU1<EC>>, ReturnType)
    where
        AddU1<EC>: Add<U1> + Unsigned + IsLessOrEqual<MEC, Output = True>,
        for<'brand> ClosureType: FnOnce(ExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        let returned_value = scope(ExposedSecret(&self.0, PhantomData));
        (
            Secret(
                // SAFETY: Since compile error prevents constructing a `Secret` with `EC` > `MEC`,
                // `zeroize()` is only called when `Secret` is maximally exposed
                // and it is not possible to call `expose_secret(...)`
                // when `Secret` is maximally exposed to access **private** `self.0` field,
                // therefore, this is safe.
                ManuallyDrop::new(unsafe { ManuallyDrop::take(&mut self.0) }),
                PhantomData,
            ),
            returned_value,
        )
    }
}

impl<T> Deref for ExposedSecret<'_, &'_ T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        self.0
    }
}

impl<T, MEC, EC> Drop for Secret<T, MEC, EC>
where
    T: Zeroize,
    MEC: Unsigned,
    EC: Add<U1> + Unsigned + IsLessOrEqual<MEC, Output = True>,
{
    #[inline(always)]
    fn drop(&mut self) {
        if EC::U64 == MEC::U64 {
            // SAFETY: Since compile error prevents constructing a `Secret` with `EC` > `MEC`,
            // `zeroize()` is only called when `Secret` is maximally exposed
            // and it is not possible to call `expose_secret(...)`
            // when `Secret` is maximally exposed to access **private** `self.0` field,
            // therefore, this is safe.
            let mut inner = unsafe { ManuallyDrop::take(&mut self.0) };
            inner.zeroize();
        }
    }
}

impl<T, MEC, EC> SecretIntoInner<T, MEC, EC> for Secret<T, MEC, EC>
where
    T: Zeroize,
    MEC: Unsigned,
    EC: Unsigned + Add<U1> + IsLessOrEqual<MEC, Output = True>,
{
    #[inline(always)]
    fn into_inner(mut self) -> T {
        // SAFETY: Since compile error prevents constructing a `Secret` with `EC` > `MEC`,
        // `zeroize()` is only called when `Secret` is maximally exposed
        // and it is not possible to call `expose_secret(...)`
        // when `Secret` is maximally exposed to access **private** `self.0` field,
        // therefore, this is safe.
        unsafe { ManuallyDrop::take(&mut self.0) }
    }
}

#[cfg(feature = "cloneable-secret")]
impl<T, MEC, EC> Clone for Secret<T, MEC, EC>
where
    T: CloneableSecret + Zeroize,
    MEC: Unsigned,
    EC: Unsigned + Add<U1> + IsLessOrEqual<MEC, Output = True>,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}
