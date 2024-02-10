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
            Err(err) => match err {
                error::ExposeSecretError::ExposeMoreThanMaximallyAllow(err) => {
                    panic!("`RTSecret` has been exposed {} times, more than what it is maximally allowed for: {} times", err.ec, err.mec)
                }
            },
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
        *ec_mut += 1;
        if *ec_mut > MEC {
            return Err(error::ExposeSecretError::ExposeMoreThanMaximallyAllow(
                error::ExposeMoreThanMaximallyAllowError {
                    mec: MEC,
                    ec: *ec_mut,
                },
            ));
        };
        Ok(scope(RTExposedSecret(&self.0, PhantomData)))
    }
}
