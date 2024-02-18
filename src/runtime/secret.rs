use core::{cell::UnsafeCell, marker::PhantomData, ops::Deref};

use crate::runtime::{
    error,
    traits::{self, __private},
};
use typenum::{IsLessOrEqual, True, Unsigned, U64};

pub struct RTSecret<T, MEC: Unsigned, SIZE: traits::MinimallyRepresentableUInt = U64>(
    T,
    UnsafeCell<<SIZE as traits::MinimallyRepresentableUInt>::Type>,
    PhantomData<MEC>,
);

pub struct RTExposedSecret<'brand, T>(T, PhantomData<fn(&'brand ()) -> &'brand ()>);

impl<'brand, T> Deref for RTExposedSecret<'brand, &'brand T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T, MEC: Unsigned, SIZE: traits::MinimallyRepresentableUInt> RTSecret<T, MEC, SIZE> {
    pub const fn new(t: T) -> Self
    where
        MEC: IsLessOrEqual<SIZE::UIntMaxValueAsType, Output = True>,
    {
        Self(t, UnsafeCell::new(SIZE::MIN), PhantomData)
    }

    pub fn new_with(f: impl FnOnce() -> T) -> Self
    where
        MEC: IsLessOrEqual<SIZE::UIntMaxValueAsType, Output = True>,
    {
        Self(f(), UnsafeCell::new(SIZE::MIN), PhantomData)
    }

    pub fn exposure_count(&self) -> &SIZE::Type {
        // SAFETY: The function only returns a shared reference (&usize) to the exposure count.
        // It does not allow mutable access to the exposure count directly.
        // This means that while external code can observe the exposure count, it cannot modify it directly.
        unsafe { &*self.1.get() }
    }
}

impl<'secret, T, MEC: Unsigned, SIZE: traits::MinimallyRepresentableUInt>
    traits::RTExposeSecret<'secret, &'secret T, SIZE> for RTSecret<T, MEC, SIZE>
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
    ) -> Result<ReturnType, error::ExposeSecretError<SIZE>>
    where
        for<'brand> ClosureType: FnOnce(RTExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        // SAFETY: All tuple fields of `RTSecret` are private, there are no setter to them.
        // `RTSecret` is also not `Sync` so it is not possible to have multithreading race condition.
        let ec_mut = unsafe { &mut *self.1.get() };
        let mec = SIZE::cast_unsigned_to_self_type::<MEC>(__private::SealedToken {});
        if *ec_mut >= mec {
            return Err(error::ExposeSecretError::ExposeMoreThanMaximallyAllow(
                error::ExposeMoreThanMaximallyAllowError { mec, ec: *ec_mut },
            ));
        };
        *ec_mut += SIZE::ONE;
        Ok(scope(RTExposedSecret(&self.0, PhantomData)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::traits::RTExposeSecret;

    #[test]
    #[should_panic(
        expected = "`RTSecret` has already been exposed 2 times, which is also the maximum number it is allowed to be exposed for."
    )]
    fn test_usize_max_expose_secret() {
        use typenum::{U2, U8};
        let mut secret_one = RTSecret::<isize, U2, U8>::new(69);
        *secret_one.1.get_mut() = u8::MAX - 1;
        #[allow(unused_assignments)]
        let mut usize_max_reached = false;

        for _ in 0..=2 {
            if secret_one.exposure_count() == &u8::MAX {
                usize_max_reached = true;
                assert!(usize_max_reached);
            };
            let _ = secret_one.expose_secret(|exposed_secret| {
                assert_eq!(*exposed_secret, 69);
            });
        }
    }
}
