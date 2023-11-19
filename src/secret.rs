// use crate::traits::ExposeSecret;
use typenum::{
    assert_type,
    consts::{U0, U1},
    op,
    type_operators::IsLess,
    Bit, IsGreater, Same, True, Unsigned, B0, B1,
};
use zeroize::Zeroize;

pub type AddU1<A> = <A as core::ops::Add<U1>>::Output;

pub struct Secret<
    T: Zeroize,
    MEC: Unsigned,
    EC: core::ops::Add<typenum::U1> + typenum::IsLess<MEC> + Unsigned = U0,
>(
    T,
    core::marker::PhantomData<MEC>,
    core::marker::PhantomData<EC>,
);

pub struct ExposedSecret<'brand, T: Zeroize, MEC: Unsigned, EC: Unsigned>(
    T,
    ::core::marker::PhantomData<fn(&'brand ()) -> &'brand ()>,
    ::core::marker::PhantomData<MEC>,
    ::core::marker::PhantomData<EC>,
);

impl<T: Zeroize, MEC: Unsigned> Secret<T, MEC, U0>
where
    U0: IsLess<MEC>,
{
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self(value, <_>::default(), <_>::default())
    }
}

// impl<
//         T: Zeroize,
//         MEC: Unsigned,
//         EC: core::ops::Add<typenum::U1> + Unsigned + typenum::IsLess<MEC>,
//     > ExposeSecret<T, MEC, EC> for Secret<T, MEC, EC>
// {
//     type Exposed<'brand> = ExposedSecret<'brand, T, MEC, EC>;

//     #[inline(always)]
//     fn expose_secret<ReturnType>(
//         self,
//         scope: impl FnOnce(ExposedSecret<'_, T, MEC, EC>) -> (ExposedSecret<'_, T, MEC, EC>, ReturnType),
//     ) -> (Secret<T, MEC, AddU1<EC>>, ReturnType)
//     where
//         AddU1<EC>: core::ops::Add<typenum::U1> + Unsigned + typenum::IsLess<MEC, Output = True>,
//         EC: IsLess<MEC, Output = True>,
//     {
//         let (witness, returned_value) = scope(ExposedSecret(
//             self.0,
//             <_>::default(),
//             <_>::default(),
//             <_>::default(),
//         ));
//         (
//             Secret(witness.0, <_>::default(), <_>::default()),
//             returned_value,
//         )
//     }
// }

impl<
        T: Zeroize,
        MEC: Unsigned,
        EC: core::ops::Add<typenum::U1> + Unsigned + typenum::IsLess<MEC>,
    > Secret<T, MEC, EC>
{
    #[inline(always)]
    pub fn expose_secret<ReturnType>(
        self,
        scope: impl FnOnce(ExposedSecret<'_, T, MEC, EC>) -> (ExposedSecret<'_, T, MEC, EC>, ReturnType),
    ) -> (Secret<T, MEC, AddU1<EC>>, ReturnType)
    where
        AddU1<EC>: core::ops::Add<typenum::U1> + Unsigned + typenum::IsLess<MEC>,
        EC: IsLess<MEC, Output=True>,
    {
        let (witness, returned_value) = scope(ExposedSecret(
            self.0,
            <_>::default(),
            <_>::default(),
            <_>::default(),
        ));
        (
            Secret(witness.0, <_>::default(), <_>::default()),
            returned_value,
        )
    }
}

impl<T: Zeroize, MEC: Unsigned, EC: Unsigned> ::core::ops::Deref for ExposedSecret<'_, T, MEC, EC> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.0
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
