use core::{
    cell::Cell,
    marker::PhantomData,
    ops::{Deref, Drop},
};

use crate::{
    runtime::{error, traits},
    traits::{ChooseMinimallyRepresentableUInt, __private},
};
use typenum::{IsGreater, True, Unsigned, U0};
#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

#[cfg(feature = "cloneable-secret")]
use crate::traits::CloneableSecret;

#[cfg(feature = "debug-secret")]
use crate::traits::DebugSecret;

pub struct RTSecret<
    #[cfg(feature = "zeroize")] T: Zeroize,
    #[cfg(not(feature = "zeroize"))] T,
    MEC: ChooseMinimallyRepresentableUInt + Unsigned,
>(T, Cell<<MEC as ChooseMinimallyRepresentableUInt>::Output>);

/// A wrapper type representing an exposed secret.
///
/// The `RTExposedSecret` struct is a wrapper type representing an exposed secret. It is associated with an invariant lifetime `'brand`,
/// indicating the lifetime of the wrapper type, which is strictly a subtype of the lifetime of the secret and cannot be coerced to be any other lifetime.
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
    #[inline(always)]
    pub const fn new(t: T) -> Self {
        Self(t, Cell::new(<MEC as ChooseMinimallyRepresentableUInt>::MIN))
    }

    #[inline(always)]
    pub fn new_with(f: impl FnOnce() -> T) -> Self {
        Self(
            f(),
            Cell::new(<MEC as ChooseMinimallyRepresentableUInt>::MIN),
        )
    }

    /// Retrieves the current exposure count of the secret and returns it as an unsigned integer.
    ///
    /// Note: The actual unsigned integer type returned depends on the type-level value of the type parameter `MEC`,
    /// it is the minimal representable Rust's unsigned integer type that can represent the value.
    /// e.g. if `MEC` is `typenum::consts::U67`, then the returned type is `u8`.
    #[inline(always)]
    pub fn exposure_count(&self) -> <MEC as ChooseMinimallyRepresentableUInt>::Output {
        self.1.get()
    }
}

impl<
        'secret,
        #[cfg(feature = "zeroize")] T: Zeroize,
        #[cfg(not(feature = "zeroize"))] T,
        // `IsGreater<U0, Output = True>` so that `RTSecret<T, U0>` cannot call `.expose_secret()`
        MEC: ChooseMinimallyRepresentableUInt + Unsigned + IsGreater<U0, Output = True>,
    > traits::RTExposeSecret<'secret, &'secret T, MEC> for RTSecret<T, MEC>
{
    type Exposed<'brand> = RTExposedSecret<'brand, &'brand T>
    where
        'secret: 'brand;

    #[inline(always)]
    fn expose_secret<ReturnType, ClosureType>(&self, scope: ClosureType) -> ReturnType
    where
        for<'brand> ClosureType: FnOnce(RTExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        match self.try_expose_secret(scope) {
            Ok(returned_value) => returned_value,
            Err(error::ExposeSecretError::ExposeMoreThanMaximallyAllow(err)) => {
                panic!("`RTSecret` has already been exposed for {} times, the maximum number it is allowed to be exposed for is {} times.", err.ec, err.mec)
            }
        }
    }

    #[inline(always)]
    fn try_expose_secret<ReturnType, ClosureType>(
        &self,
        scope: ClosureType,
    ) -> Result<ReturnType, error::ExposeSecretError<MEC>>
    where
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
    /// Zeroizes the secret value when dropped if the `zeroize` feature is enabled.
    fn drop(&mut self) {
        #[cfg(feature = "zeroize")]
        self.0.zeroize()
    }
}

#[cfg(feature = "cloneable-secret")]
impl<T, MEC> Clone for RTSecret<T, MEC>
where
    T: CloneableSecret,
    MEC: ChooseMinimallyRepresentableUInt + Unsigned,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

#[cfg(feature = "debug-secret")]
impl<T, MEC> core::fmt::Debug for RTSecret<T, MEC>
where
    T: DebugSecret,
    MEC: ChooseMinimallyRepresentableUInt + Unsigned,
{
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("RTSecret<")?;
        T::debug_secret(f)?;
        f.write_str(">")
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
