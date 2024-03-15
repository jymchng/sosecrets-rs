use core::{
    cell::Cell,
    convert::Infallible,
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, Drop},
};

use crate::{
    runtime::{error, traits},
    traits::{ChooseMinimallyRepresentableUInt, __private},
    types::NumericalZeroSizedType,
};
use typenum::{IsGreater, True, Unsigned, U0};
#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

#[cfg(feature = "cloneable-secret")]
use crate::traits::CloneableSecret;

#[cfg(feature = "debug-secret")]
use crate::traits::DebugSecret;

/// A runtime secret with optional zeroization for the type `T` and exposure count tracking. It is the runtime version of `Secret<T, MEC, EC>`.
pub struct RTSecret<
    #[cfg(feature = "zeroize")] T: Zeroize,
    #[cfg(not(feature = "zeroize"))] T,
    MEC: ChooseMinimallyRepresentableUInt,
>(
    /// `T` is the type of the value that is meant to be kept as a secret,
    T,
    /// The type of the exposure counter, can be either `u8`, `u16`, `u32` or `u64`.
    Cell<<MEC as ChooseMinimallyRepresentableUInt>::Output>,
);

/// A wrapper type representing an exposed secret.
///
/// The `RTExposedSecret` struct is a wrapper type representing an exposed secret.
/// It holds an annotated (`'brand`) [invariant](https://doc.rust-lang.org/nomicon/subtyping.html#variance) lifetime, indicating the lifetime of the wrapper type, which is strictly a subtype of the lifetime of the secret and cannot be coerced to be any other lifetime.
pub struct RTExposedSecret<'brand, T>(T, PhantomData<fn(&'brand ()) -> &'brand ()>);

/// A convenience alias for `RTSecret` with a secret of type `T` that does **not** conduct any exposure count checking, i.e. the secret can be exposed infinitely many times.
/// It is meant to function almost identically to `secrecy::Secret`, except that the signature of `.expose_secret(...)` method is different.
pub type SecrecySecret<T> = RTSecret<T, NumericalZeroSizedType>;

impl<'secret, #[cfg(feature = "zeroize")] T: Zeroize, #[cfg(not(feature = "zeroize"))] T>
    traits::RTExposeSecret<'secret, &'secret T> for RTSecret<T, NumericalZeroSizedType>
{
    type Error = Infallible;

    type Exposed<'brand> = RTExposedSecret<'brand, &'brand T>
    where
        'secret: 'brand;

    /// Exposes the secret **without** any runtime checking that the exposure count is not more than the maximally allowed exposure count represented by the type parameter `MEC`.
    /// Note: It is impossible to return the 'exposed secret' as the return value of the closure.
    ///
    /// Example:
    /// ```rust
    /// use sosecrets_rs::{
    ///     prelude::{typenum::U2, SecrecySecret, RTSecret},
    ///     runtime::traits::RTExposeSecret,
    /// };
    /// #[cfg(feature = "zeroize")]
    /// use zeroize::Zeroize;
    ///
    /// struct A {
    ///     inner: i32,
    /// }
    ///
    /// #[cfg(feature = "zeroize")]
    /// impl Zeroize for A {
    ///     fn zeroize(&mut self) {
    ///         self.inner.zeroize()
    ///     }
    /// }
    ///
    /// let secret_one = SecrecySecret::<A>::new(A { inner: 69 });
    /// let returned_value = secret_one.expose_secret(|exposed_secret| A { inner: (*exposed_secret).inner + 1});
    /// assert_eq!(returned_value.inner, 70);
    /// ```
    ///
    /// Example (this does **NOT** compile):
    /// ```compile_fail
    /// use sosecrets_rs::{
    ///     prelude::{typenum::U2, SecrecySecret, RTSecret},
    ///     runtime::traits::RTExposeSecret,
    /// };
    /// #[cfg(feature = "zeroize")]
    /// use zeroize::Zeroize;
    ///
    /// struct A {
    ///     inner: i32,
    /// }
    ///
    /// #[cfg(feature = "zeroize")]
    /// impl Zeroize for A {
    ///     fn zeroize(&mut self) {
    ///         self.inner.zeroize()
    ///     }
    /// }
    ///
    /// let secret_one = SecrecySecret::<A>::new(A { inner: 69 });
    /// let _ = secret_one.expose_secret(|exposed_secret| exposed_secret);
    /// let _ = secret_one.expose_secret(|exposed_secret| *exposed_secret); // Only if T is not `Copy`
    /// ```
    ///
    /// # Parameters
    /// - `self`.
    /// - `scope`: A closure that takes the exposed secret and returns a value of the `ReturnType`.
    /// # Returns
    /// A value of type `ReturnType` which is the type of the returned value from the closure named `scope`.
    #[inline(always)]
    fn expose_secret<ReturnType, ClosureType>(&self, scope: ClosureType) -> ReturnType
    where
        for<'brand> ClosureType: FnOnce(RTExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        scope(RTExposedSecret(&self.0, PhantomData))
    }

    /// Exposes the secret **without** any runtime checking that the exposure count is not more than the maximally allowed exposure count represented by the type parameter `MEC`.
    /// Note: It is impossible to return the 'exposed secret' as the return value of the closure.
    ///
    /// Example:
    /// ```rust
    /// use sosecrets_rs::{
    ///     prelude::{typenum::U2, SecrecySecret, RTSecret},
    ///     runtime::traits::RTExposeSecret,
    /// };
    /// #[cfg(feature = "zeroize")]
    /// use zeroize::Zeroize;
    ///
    /// struct A {
    ///     inner: i32,
    /// }
    ///
    /// #[cfg(feature = "zeroize")]
    /// impl Zeroize for A {
    ///     fn zeroize(&mut self) {
    ///         self.inner.zeroize()
    ///     }
    /// }
    ///
    /// let secret_one = SecrecySecret::<A>::new(A { inner: 69 });
    /// let returned_value = secret_one.try_expose_secret(|exposed_secret| A { inner: (*exposed_secret).inner + 1});
    /// assert!(returned_value.is_ok());
    /// ```
    ///
    /// Example (this does **NOT** compile):
    /// ```compile_fail
    /// use sosecrets_rs::{
    ///     prelude::typenum::U2,
    ///     runtime::{secret::RTSecret, traits::RTExposeSecret},
    /// };
    /// #[cfg(feature = "zeroize")]
    /// use zeroize::Zeroize;
    ///
    /// struct A {
    ///     inner: i32,
    /// }
    ///
    /// #[cfg(feature = "zeroize")]
    /// impl Zeroize for A {
    ///     fn zeroize(&mut self) {
    ///         self.inner.zeroize()
    ///     }
    /// }
    ///
    /// let secret_one = SecrecySecret::<A>::new(A { inner: 69 });
    /// let _ = secret_one.try_expose_secret(|exposed_secret| exposed_secret);
    /// let _ = secret_one.try_expose_secret(|exposed_secret| *exposed_secret); // Only if T is not `Copy`
    /// ```
    ///
    /// # Parameters
    /// - `self`.
    /// - `scope`: A closure that takes the exposed secret and returns a value of the `ReturnType`.
    ///
    /// # Returns
    /// An `Ok` variant containing the value of type `ReturnType` which is the type of the returned value from the closure named `scope`.
    /// This function can **never** fail because no check is done.
    #[inline(always)]
    fn try_expose_secret<ReturnType, ClosureType>(
        &self,
        scope: ClosureType,
    ) -> Result<ReturnType, Infallible>
    where
        for<'brand> ClosureType: FnOnce(RTExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        Ok(scope(RTExposedSecret(&self.0, PhantomData)))
    }
}

impl<'brand, T> Deref for RTExposedSecret<'brand, &'brand T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<
        #[cfg(feature = "zeroize")] T: Zeroize,
        #[cfg(not(feature = "zeroize"))] T,
        MEC: ChooseMinimallyRepresentableUInt,
    > RTSecret<T, MEC>
{
    /// Creates a new `RTSecret` with the provided secret value `t`.
    ///
    /// # Parameters
    /// - `t`: The secret value.
    ///
    /// # Returns
    /// The newly created `RTSecret`.
    #[inline(always)]
    pub const fn new(t: T) -> Self {
        Self(
            t,
            Cell::new(<MEC as ChooseMinimallyRepresentableUInt>::ZERO),
        )
    }

    /// Creates a new `RTSecret` with the provided secret value returned by the closure `f`.
    ///
    /// # Parameters
    /// - `f`: A closure that returns the secret value.
    ///
    /// # Returns
    /// The newly created `RTSecret`.
    #[inline(always)]
    pub fn new_with(f: impl FnOnce() -> T) -> Self {
        Self(
            f(),
            Cell::new(<MEC as ChooseMinimallyRepresentableUInt>::ZERO),
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

    #[inline(always)]
    fn can_expose(&self) -> bool
    where
        MEC: typenum::Unsigned,
    {
        let ec = self.1.get();
        let mec = MEC::cast_unsigned_to_self_type::<MEC>(__private::SealedToken {});
        if ec >= mec {
            return false;
        };
        self.1.set(ec + MEC::ONE);
        true
    }
}

impl<
        'secret,
        #[cfg(feature = "zeroize")] T: Zeroize,
        #[cfg(not(feature = "zeroize"))] T,
        // `IsGreater<U0, Output = True>` so that `RTSecret<T, U0>` cannot call `.expose_secret()`
        MEC: ChooseMinimallyRepresentableUInt + Unsigned + IsGreater<U0, Output = True> + Debug,
    > traits::RTExposeSecret<'secret, &'secret T> for RTSecret<T, MEC>
{
    type Error = error::ExposeSecretError<MEC>;

    type Exposed<'brand> = RTExposedSecret<'brand, &'brand T>
    where
        'secret: 'brand;

    /// Exposes the secret with runtime checking that the exposure count is not more than the maximally allowed exposure count represented by the type parameter `MEC`.
    /// Note: It is impossible to return the 'exposed secret' as the return value of the closure.
    ///
    /// Example:
    /// ```rust
    /// use sosecrets_rs::{
    ///     prelude::typenum::U2,
    ///     runtime::{secret::RTSecret, traits::RTExposeSecret},
    /// };
    /// #[cfg(feature = "zeroize")]
    /// use zeroize::Zeroize;
    ///
    /// struct A {
    ///     inner: i32,
    /// }
    ///
    /// #[cfg(feature = "zeroize")]
    /// impl Zeroize for A {
    ///     fn zeroize(&mut self) {
    ///         self.inner.zeroize()
    ///     }
    /// }
    ///
    /// let secret_one = RTSecret::<A, U2>::new(A { inner: 69 });
    /// let returned_value = secret_one.expose_secret(|exposed_secret| A { inner: (*exposed_secret).inner + 1});
    /// assert_eq!(returned_value.inner, 70);
    /// ```
    ///
    /// Example:
    /// ```compile_fail
    /// use sosecrets_rs::{
    ///     prelude::typenum::U2,
    ///     runtime::{secret::RTSecret, traits::RTExposeSecret},
    /// };
    /// #[cfg(feature = "zeroize")]
    /// use zeroize::Zeroize;
    ///
    /// struct A {
    ///     inner: i32,
    /// }
    ///
    /// #[cfg(feature = "zeroize")]
    /// impl Zeroize for A {
    ///     fn zeroize(&mut self) {
    ///         self.inner.zeroize()
    ///     }
    /// }
    ///
    /// let secret_one = RTSecret::<A, U2>::new(A { inner: 69 });
    /// let _ = secret_one.expose_secret(|exposed_secret| exposed_secret);
    /// let _ = secret_one.expose_secret(|exposed_secret| *exposed_secret); // Only if T is not `Copy`
    /// ```
    ///
    /// # Parameters
    /// - `self`.
    /// - `scope`: A closure that takes the exposed secret and returns a value of the `ReturnType`.
    ///
    /// # Panics
    /// This function panics only if the secret is exposed more than the maximally allowed exposure count represented by the type parameter `MEC`.
    ///
    /// # Returns
    /// A value of type `ReturnType` which is the type of the returned value from the closure named `scope`.
    #[inline(always)]
    fn expose_secret<ReturnType, ClosureType>(&self, scope: ClosureType) -> ReturnType
    where
        for<'brand> ClosureType: FnOnce(RTExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        if self.can_expose() {
            return scope(RTExposedSecret(&self.0, PhantomData));
        } else {
            let ec = self.exposure_count();
            let mec = MEC::cast_unsigned_to_self_type::<MEC>(__private::SealedToken {});
            panic!("`RTSecret` has already been exposed for {} times, the maximum number it is allowed to be exposed for is {} times.", ec, mec)
        }
    }

    /// Return the `Result` containing `Ok(scope(exposed_secret))`, with runtime checking that the exposure count is not more than the maximally allowed exposure count represented by the type parameter `MEC`.
    /// Note: It is impossible to return the 'exposed secret' as the return value of the closure.
    ///
    /// Example:
    /// ```rust
    /// use sosecrets_rs::{
    ///     prelude::{typenum::U2, RTSecret},
    ///     runtime::traits::RTExposeSecret,
    /// };
    /// #[cfg(feature = "zeroize")]
    /// use zeroize::Zeroize;
    ///
    /// struct A {
    ///     inner: i32,
    /// }
    ///
    /// #[cfg(feature = "zeroize")]
    /// impl Zeroize for A {
    ///     fn zeroize(&mut self) {
    ///         self.inner.zeroize()
    ///     }
    /// }
    ///
    /// let secret_one = RTSecret::<A, U2>::new(A { inner: 69 });
    /// let returned_value = secret_one.try_expose_secret(|exposed_secret| A { inner: (*exposed_secret).inner + 1});
    /// assert!(returned_value.is_ok());
    /// ```
    ///
    /// Example (this example will **not** compile):
    /// ```compile_fail
    /// use sosecrets_rs::{
    ///     prelude::typenum::U2,
    ///     runtime::{secret::RTSecret, traits::RTExposeSecret},
    /// };
    /// #[cfg(feature = "zeroize")]
    /// use zeroize::Zeroize;
    ///
    /// struct A {
    ///     inner: i32,
    /// }
    ///
    /// #[cfg(feature = "zeroize")]
    /// impl Zeroize for A {
    ///     fn zeroize(&mut self) {
    ///         self.inner.zeroize()
    ///     }
    /// }
    ///
    /// let secret_one = RTSecret::<A, U2>::new(A { inner: 69 });
    /// let _ = secret_one.try_expose_secret(|exposed_secret| exposed_secret);
    /// let _ = secret_one.try_expose_secret(|exposed_secret| *exposed_secret); // Only if T is not `Copy`
    /// ```
    ///
    /// # Parameters
    /// - `self`.
    /// - `scope`: A closure that takes the exposed secret and returns a value of the `ReturnType`.
    ///
    ///
    /// # Returns
    /// - `Ok`: The value returned by the closure.
    /// - `Err`: If the exposure count exceeds the maximum allowed, returns an `ExposeSecretError`.
    #[inline(always)]
    fn try_expose_secret<ReturnType, ClosureType>(
        &self,
        scope: ClosureType,
    ) -> Result<ReturnType, error::ExposeSecretError<MEC>>
    where
        for<'brand> ClosureType: FnOnce(RTExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        if self.can_expose() {
            Ok(scope(RTExposedSecret(&self.0, PhantomData)))
        } else {
            let ec = self.exposure_count();
            let mec = MEC::cast_unsigned_to_self_type::<MEC>(__private::SealedToken {});
            Err(error::ExposeSecretError::ExposeMoreThanMaximallyAllow(
                error::ExposeMoreThanMaximallyAllowError { mec, ec },
            ))
        }
    }
}

impl<
        #[cfg(feature = "zeroize")] T: Zeroize,
        #[cfg(not(feature = "zeroize"))] T,
        MEC: ChooseMinimallyRepresentableUInt,
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
