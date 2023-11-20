use std::marker::PhantomData;

use crate::traits::ExposeSecret;
use typenum::{
    consts::{U0, U1},
    type_operators::IsLess,
    Sum, True, Unsigned,
};
use zeroize::{DefaultIsZeroes, Zeroize};

pub type AddU1<A> = <A as core::ops::Add<U1>>::Output;

pub struct Secret<
    T: Zeroize,
    MEC: Unsigned,
    EC: core::ops::Add<typenum::U1> + typenum::IsLess<MEC> + Unsigned = U0,
>(T, core::marker::PhantomData<(MEC, EC)>);

pub struct ExposedSecret<'brand, T>(T, ::core::marker::PhantomData<fn(&'brand ()) -> &'brand ()>);

impl<T: Zeroize, MEC: Unsigned> Secret<T, MEC, U0>
where
    U0: IsLess<MEC>,
{
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self(value, PhantomData)
    }
}

impl<
        'max,
        T: Zeroize,
        MEC: Unsigned,
        EC: core::ops::Add<typenum::U1> + Unsigned + typenum::IsLess<MEC>,
    > ExposeSecret<'max, &'max T, MEC, EC> for Secret<T, MEC, EC>
{
    type Exposed<'brand> = ExposedSecret<'brand, &'brand T>
    where
        'max: 'brand;

    type Next = Secret<T, MEC, Sum<EC, U1>>
    where
        EC: core::ops::Add<U1> + Unsigned + typenum::IsLess<MEC>,
        Sum<EC, U1>: Unsigned + IsLess<MEC> + core::ops::Add<typenum::U1>,
        T: Zeroize;

    #[inline(always)]
    fn expose_secret<ReturnType, ClosureType>(
        self,
        scope: ClosureType,
    ) -> (Secret<T, MEC, AddU1<EC>>, ReturnType)
    where
        AddU1<EC>: core::ops::Add<typenum::U1> + Unsigned + typenum::IsLess<MEC>,
        EC: IsLess<MEC, Output = True>,
        for<'brand> ClosureType: FnOnce(ExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        let returned_value = scope(ExposedSecret(&self.0, PhantomData));
        (Secret(self.0, PhantomData), returned_value)
    }
}

impl<T: Zeroize + DefaultIsZeroes> ::core::ops::Deref for ExposedSecret<'_, &'_ T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        self.0
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use typenum::consts::{U1, U2, U3, U4, U5};
    #[derive(Debug)]
    struct UseSecret<T> {
        inner: T,
    }
    impl<T> UseSecret<T> {
        fn new(value: T) -> Self {
            Self { inner: value }
        }
    }

    #[test]
    fn test_expose_secret() {
        let new_secret: Secret<String, U2> = Secret::new("mySecret".to_string());

        let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
            let returned_value = UseSecret::new((*exposed_secret).to_string());
            (exposed_secret, returned_value)
        });
        assert_eq!("mySecret", &returned_value.inner);

        let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
            let returned_value = UseSecret::new((*exposed_secret).to_string());
            (exposed_secret, returned_value)
        });
        assert_eq!("mySecret", &returned_value.inner);

        // let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        //     let returned_value = UseSecret::new((*exposed_secret).to_string());
        //     (exposed_secret, returned_value)
        // });
        // assert_eq!("mySecret", returned_value.inner);
    }

    #[test]
    fn test_expose_secret_2() {
        let new_secret: Secret<_, U2> = Secret::new(69);

        let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
            let returned_value = UseSecret::new(*exposed_secret);
            (exposed_secret, returned_value)
        });
        assert_eq!(69, returned_value.inner);

        let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
            let returned_value = UseSecret::new(*exposed_secret);
            (exposed_secret, returned_value)
        });
        assert_eq!(69, returned_value.inner);

        // let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        //     let returned_value = UseSecret::new(*exposed_secret);
        //     (exposed_secret, returned_value)
        // });
        // assert_eq!(69, returned_value.inner);
    }
}
