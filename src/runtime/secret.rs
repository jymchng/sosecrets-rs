use core::{cell::UnsafeCell, marker::PhantomData, ops::Deref};

use crate::runtime::{error, traits};
pub struct RTSecret<T, const MEC: usize>(T, UnsafeCell<usize>);
pub struct RTExposedSecret<'brand, T>(T, PhantomData<fn(&'brand ()) -> &'brand ()>);

impl<'brand, T> Deref for RTExposedSecret<'brand, &'brand T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T, const MEC: usize> RTSecret<T, MEC> {
    pub const fn new(value: T) -> Self {
        Self(value, UnsafeCell::new(0))
    }

    pub fn new_with(f: impl FnOnce() -> T) -> Self {
        Self(f(), UnsafeCell::new(0))
    }

    pub fn exposure_count(&self) -> usize {
        unsafe { *self.1.get() }
    }
}

impl<'secret, T, const MEC: usize> traits::RTExposeSecret<'secret, &'secret T>
    for RTSecret<T, MEC>
{
    type Exposed<'brand> = RTExposedSecret<'brand, &'brand T>
    where
        'secret: 'brand;

    fn expose_secret<ReturnType, ClosureType>(&self, scope: ClosureType) -> ReturnType
    where
        for<'brand> ClosureType: FnOnce(RTExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        match self.try_expose_secret(scope) {
            Ok(returned_value) => returned_value,
            Err(error::ExposeSecretError::ExposeMoreThanMaximallyAllow(err)) => {
                panic!("`RTSecret` has already been exposed {} times, which is also the maximum number it is allowed to be exposed for.", err.mec)
            }
        }
    }

    fn try_expose_secret<ReturnType, ClosureType>(
        &self,
        scope: ClosureType,
    ) -> Result<ReturnType, error::ExposeSecretError>
    where
        for<'brand> ClosureType: FnOnce(RTExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        // SAFETY: All tuple fields of `RTSecret` are private, there are no getter / setter to them.
        // `RTSecret` is also not `Sync` so it is not possible to have multithreading race condition.
        let ec_mut = unsafe { &mut *self.1.get() };
        if *ec_mut >= MEC {
            return Err(error::ExposeSecretError::ExposeMoreThanMaximallyAllow(
                error::ExposeMoreThanMaximallyAllowError {
                    mec: MEC,
                    ec: *ec_mut,
                },
            ));
        };
        *ec_mut += 1;
        Ok(scope(RTExposedSecret(&self.0, PhantomData)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::traits::RTExposeSecret;

    #[test]
    #[should_panic(
        expected = "`RTSecret` has already been exposed 18446744073709551615 times, which is also the maximum number it is allowed to be exposed for."
    )]
    fn test_usize_max_expose_secret() {
        extern crate std;
        let mut secret_one = RTSecret::<isize, { usize::MAX }>::new(69);
        *secret_one.1.get_mut() = usize::MAX - 1;
        #[allow(unused_assignments)]
        let mut usize_max_reached = false;

        for _ in 0..=2 {
            if secret_one.exposure_count() == usize::MAX {
                usize_max_reached = true;
                assert!(usize_max_reached);
            };
            let _ = secret_one.expose_secret(|exposed_secret| {
                assert_eq!(*exposed_secret, 69);
            });
        }
    }
}
