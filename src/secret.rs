use core::{
    marker::PhantomData,
    ops::{Add, Deref},
};

use crate::traits::{CloneableSecret, ExposeSecret};
use typenum::{
    consts::{U0, U1},
    type_operators::IsLess,
    Sum, True, Unsigned,
};
use zeroize::Zeroize;

pub type AddU1<A> = <A as core::ops::Add<U1>>::Output;

pub struct Secret<
    T: Zeroize,
    MEC: Unsigned,
    EC: core::ops::Add<typenum::U1> + typenum::IsLess<MEC> + Unsigned = U0,
>(T, PhantomData<(MEC, EC)>);

pub struct ExposedSecret<'brand, T>(T, PhantomData<fn(&'brand ()) -> &'brand ()>);

impl<T: Zeroize, MEC: Unsigned> Secret<T, MEC, U0>
where
    U0: IsLess<MEC>,
{
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self(value, PhantomData)
    }
}

impl<'max, T: Zeroize, MEC: Unsigned, EC: Add<U1> + Unsigned + IsLess<MEC>>
    ExposeSecret<'max, &'max T, MEC, EC> for Secret<T, MEC, EC>
{
    type Exposed<'brand> = ExposedSecret<'brand, &'brand T>
    where
        'max: 'brand;

    type Next = Secret<T, MEC, Sum<EC, U1>>
    where
        EC: Add<U1> + Unsigned + IsLess<MEC>,
        Sum<EC, U1>: Unsigned + IsLess<MEC> + Add<U1>,
        T: Zeroize;

    #[inline(always)]
    fn expose_secret<ReturnType, ClosureType>(
        self,
        scope: ClosureType,
    ) -> (Secret<T, MEC, AddU1<EC>>, ReturnType)
    where
        AddU1<EC>: Add<U1> + Unsigned + IsLess<MEC>,
        EC: IsLess<MEC, Output = True>,
        for<'brand> ClosureType: FnOnce(ExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        let returned_value = scope(ExposedSecret(&self.0, PhantomData));
        (Secret(self.0, PhantomData), returned_value)
    }
}

impl<T, MEC, EC> CloneableSecret<T, MEC, EC> for Secret<T, MEC, EC>
where
    T: Clone + Zeroize,
    MEC: Unsigned,
    EC: Unsigned + Add<U1> + IsLess<MEC>,
{
    fn clone_secret(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<T> Deref for ExposedSecret<'_, &'_ T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        self.0
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use typenum::consts::U2;
    #[derive(Debug)]
    struct UseSecret<T> {
        inner: T,
    }
    impl<T> UseSecret<T> {
        fn new(value: T) -> Self {
            Self { inner: value }
        }
    }
}
