use core::{
    marker::PhantomData,
    mem::{forget, ManuallyDrop},
    ops::{Add, Deref, Drop},
};

use crate::traits::ExposeSecret;
use typenum::{IsLessOrEqual, Sum, True, Unsigned, U0, U1};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

#[cfg(feature = "cloneable-secret")]
use crate::traits::CloneableSecret;

type AddU1<A> = <A as core::ops::Add<U1>>::Output;

pub struct Secret<
    #[cfg(feature = "zeroize")] T: Zeroize,
    #[cfg(not(feature = "zeroize"))] T,
    MEC: Unsigned,
    EC: Add<U1> + IsLessOrEqual<MEC, Output = True> + Unsigned = U0,
>(ManuallyDrop<T>, PhantomData<(MEC, EC)>);

pub struct ExposedSecret<'brand, T>(T, PhantomData<fn(&'brand ()) -> &'brand ()>);

impl<#[cfg(feature = "zeroize")] T: Zeroize, #[cfg(not(feature = "zeroize"))] T, MEC: Unsigned>
    Secret<T, MEC, U0>
where
    U0: IsLessOrEqual<MEC, Output = True>,
{
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self(ManuallyDrop::new(value), PhantomData)
    }

    #[inline(always)]
    pub fn new_with<ClosureType>(closure: ClosureType) -> Self
    where
        ClosureType: FnOnce() -> T,
    {
        Self(ManuallyDrop::new(closure()), PhantomData)
    }
}

impl<
        'max,
        #[cfg(feature = "zeroize")] T: Zeroize,
        #[cfg(not(feature = "zeroize"))] T,
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
        Sum<EC, U1>: Unsigned + IsLessOrEqual<MEC, Output = True> + Add<U1>;

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
        // SAFETY: Since compile error prevents constructing a `Secret` with `EC` > `MEC`,
        // `zeroize()` is only called when `Secret` is maximally exposed
        // and it is not possible to call `expose_secret(...)`
        // when `Secret` is maximally exposed to access **private** `self.0` field,
        // therefore, this is safe.
        let inner = ManuallyDrop::new(unsafe { ManuallyDrop::take(&mut self.0) });
        forget(self);
        (Secret(inner, PhantomData), returned_value)
    }
}

impl<T> Deref for ExposedSecret<'_, &'_ T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        self.0
    }
}

impl<#[cfg(feature = "zeroize")] T: Zeroize, #[cfg(not(feature = "zeroize"))] T, MEC, EC> Drop
    for Secret<T, MEC, EC>
where
    MEC: Unsigned,
    EC: Add<U1> + Unsigned + IsLessOrEqual<MEC, Output = True>,
{
    #[inline(always)]
    fn drop(&mut self) {
        // SAFETY: Since compile error prevents constructing a `Secret` with `EC` > `MEC`,
        // `zeroize()` is only called when `Secret` is maximally exposed
        // and it is not possible to call `expose_secret(...)`
        // when `Secret` is maximally exposed to access **private** `self.0` field,
        // therefore, this is safe.
        let mut _inner = unsafe { ManuallyDrop::take(&mut self.0) };
        #[cfg(feature = "zeroize")]
        _inner.zeroize();
    }
}

#[cfg(feature = "cloneable-secret")]
impl<T, MEC, EC> Clone for Secret<T, MEC, EC>
where
    T: CloneableSecret,
    MEC: Unsigned,
    EC: Unsigned + Add<U1> + IsLessOrEqual<MEC, Output = True>,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}
