#![feature(generic_arg_infer)]
#![feature(generic_const_exprs)]
use crate::generic_const_predicate;

pub struct Secret<T, const MEC: usize, const EC: usize = 0>(T);

pub struct ExposedSecret<'brand, T, const MEC: usize, const EC: usize = 0>(
    T,
    ::core::marker::PhantomData<fn(&'brand ()) -> &'brand ()>,
);

impl<T, const MEC: usize, const EC: usize> Secret<T, MEC, EC> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn expose_secret<ReturnType>(
        self,
        scope: impl FnOnce(ExposedSecret<'_, T, MEC, EC>) -> (ExposedSecret<'_, T, MEC, EC>, ReturnType),
    ) -> (Secret<T, MEC, { EC + 1 }>, ReturnType)
    where
        generic_const_predicate!(MEC > EC):,
        Secret<T, MEC, { EC + 1 }>:,
    {
        let (witness, returned_value) = scope(ExposedSecret(self.0, <_>::default()));
        (Secret::new(witness.0), returned_value)
    }
}

impl<T, const MEC: usize, const EC: usize> ::core::ops::Deref for ExposedSecret<'_, T, MEC, EC> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.0
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_expose_secret() {
        use super::*;
        #[derive(Debug)]
        struct UseSecret {
            inner: String,
        }
        impl UseSecret {
            fn new(value: impl AsRef<str>) -> Self {
                Self {
                    inner: value.as_ref().to_string(),
                }
            }
        }

        let new_secret: Secret<&str, 2, 0> = Secret::new("mySecret");
        let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
            let returned_value = UseSecret::new(*exposed_secret);
            (exposed_secret, returned_value)
        });
        assert_eq!("mySecret", returned_value.inner);
        let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
            let returned_value = UseSecret::new(*exposed_secret);
            (exposed_secret, returned_value)
        });
        assert_eq!("mySecret", returned_value.inner);
        // let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        //     let returned_value = UseSecret::new(*exposed_secret);
        //     (exposed_secret, returned_value)
        // });
        // assert_eq!("mySecret", returned_value.inner);
    }

    #[test]
    fn test_expose_secret_2() {
        use super::*;
        #[derive(Debug)]
        struct UseSecret {
            inner: i32,
        }
        impl UseSecret {
            fn new(value: i32) -> Self {
                Self { inner: value }
            }
        }

        let new_secret: Secret<_, 2> = Secret::new(69);
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
    }
}
