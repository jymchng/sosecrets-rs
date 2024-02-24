use core::{
    cell::Cell,
    marker::PhantomData,
    ops::{Deref, Drop},
};

use crate::{
    runtime::{common::Unchecked, error, traits},
    traits::{ChooseMinimallyRepresentableUInt, __private},
};
use typenum::{IsGreater, True, Unsigned, U0};
#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

pub struct RTSecret<
    #[cfg(feature = "zeroize")] T: Zeroize,
    #[cfg(not(feature = "zeroize"))] T,
    MEC: ChooseMinimallyRepresentableUInt + Unsigned,
>(T, Cell<<MEC as ChooseMinimallyRepresentableUInt>::Output>);

pub struct RTExposedSecret<'brand, T>(T, PhantomData<fn(&'brand ()) -> &'brand ()>);

impl<'brand, T> Deref for RTExposedSecret<'brand, &'brand T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<
        #[cfg(feature = "zeroize")] T: Zeroize,
        #[cfg(not(feature = "zeroize"))] T,
        MEC: ChooseMinimallyRepresentableUInt + Unsigned,
    > RTSecret<T, MEC>
{
    pub const fn new(t: T) -> Self {
        Self(t, Cell::new(<MEC as ChooseMinimallyRepresentableUInt>::MIN))
    }

    pub fn new_with(f: impl FnOnce() -> T) -> Self {
        Self(
            f(),
            Cell::new(<MEC as ChooseMinimallyRepresentableUInt>::MIN),
        )
    }

    pub fn exposure_count(&self) -> <MEC as ChooseMinimallyRepresentableUInt>::Output {
        self.1.get()
    }

    // Should be a trait impl
    // pub fn expose_secret_unchecked<ReturnType, ClosureType>(&self, scope: ClosureType) -> ReturnType
    // where
    //     MEC: IsEqual<Unchecked, Output = True>,
    //     for<'brand> ClosureType: FnOnce(RTExposedSecret<'brand, &'brand T>) -> ReturnType,
    // {
    //     scope(RTExposedSecret(&self.0, PhantomData))
    // }
}

impl<
        'secret,
        #[cfg(feature = "zeroize")] T: Zeroize,
        #[cfg(not(feature = "zeroize"))] T,
        MEC: ChooseMinimallyRepresentableUInt + Unsigned + IsGreater<U0, Output = True>,
    > traits::RTExposeSecret<'secret, &'secret T, MEC> for RTSecret<T, MEC>
{
    type Exposed<'brand> = RTExposedSecret<'brand, &'brand T>
    where
        'secret: 'brand;

    fn expose_secret<ReturnType, ClosureType>(&self, scope: ClosureType) -> ReturnType
    where
        MEC: IsGreater<Unchecked, Output = True>,
        for<'brand> ClosureType: FnOnce(RTExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        match self.try_expose_secret(scope) {
            Ok(returned_value) => returned_value,
            Err(error::ExposeSecretError::ExposeMoreThanMaximallyAllow(err)) => {
                panic!("`RTSecret` has already been exposed for {} times, the maximum number it is allowed to be exposed for is {} times.", err.ec, err.mec)
            }
        }
    }

    fn try_expose_secret<ReturnType, ClosureType>(
        &self,
        scope: ClosureType,
    ) -> Result<ReturnType, error::ExposeSecretError<MEC>>
    where
        MEC: IsGreater<Unchecked, Output = True>,
        for<'brand> ClosureType: FnOnce(RTExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        let ec_mut = self.1.get();
        let mec = MEC::cast_unsigned_to_self_type::<MEC>(__private::SealedToken {});
        if ec_mut >= mec {
            return Err(error::ExposeSecretError::ExposeMoreThanMaximallyAllow(
                error::ExposeMoreThanMaximallyAllowError { mec, ec: ec_mut },
            ));
        };
        self.1.set(ec_mut + MEC::ONE);
        Ok(scope(RTExposedSecret(&self.0, PhantomData)))
    }
}

impl<
        #[cfg(feature = "zeroize")] T: Zeroize,
        #[cfg(not(feature = "zeroize"))] T,
        MEC: ChooseMinimallyRepresentableUInt + Unsigned,
    > Drop for RTSecret<T, MEC>
{
    fn drop(&mut self) {
        #[cfg(feature = "zeroize")]
        self.0.zeroize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::traits::RTExposeSecret;

    #[test]
    #[should_panic(
        expected = "`RTSecret` has already been exposed for 255 times, the maximum number it is allowed to be exposed for is 255 times."
    )]
    fn test_usize_max_expose_secret() {
        use typenum::U255;
        let mut secret_one = RTSecret::<isize, U255>::new(69);
        *secret_one.1.get_mut() = u8::MAX - 6;

        for _ in 0..=5 {
            let _ = secret_one.expose_secret(|exposed_secret| {
                assert_eq!(*exposed_secret, 69);
            });
        }

        assert_eq!(secret_one.exposure_count(), u8::MAX);

        let _ = secret_one.expose_secret(|exposed_secret| {
            assert_eq!(*exposed_secret, 69);
        });
    }
}
