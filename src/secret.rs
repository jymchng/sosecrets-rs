//! # `sosecrets-rs`
//! `sosecret-rs` is a Rust crate providing a Secret type for managing secret values with exposure control.
//! It aims to enhance security by allowing controlled exposure of sensitive information.
//!
//! # Features
//! Exposure Control: Secret values can only be exposed a limited number of times, preventing unintentional information leaks. This is guaranteed at compile time.
//! Zeroization: If configured with the zeroize feature, secrets are zeroized upon reaching their maximum exposure count.
//! Cloneable Secrets: With the cloneable-secret feature, Secret values can be cloned if the underlying type implements the CloneableSecret trait.
//! Debugging Secrets: The debug-secret feature enables the debugging of Secret values if the underlying type implements the DebugSecret trait.

use core::{
    marker::PhantomData,
    mem::{forget, ManuallyDrop},
    ops::{Deref, Drop, Sub},
};

use crate::traits::ExposeSecret;
pub use typenum;
use typenum::{IsGreaterOrEqual, True, Unsigned, U0, U1};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

#[cfg(feature = "cloneable-secret")]
use crate::traits::CloneableSecret;

#[cfg(feature = "debug-secret")]
use crate::traits::DebugSecret;

pub(crate) type Sub1<K> = <K as Sub<U1>>::Output;

/// The `Secret` struct represents a secure container for managing sensitive values with built-in exposure control.
///
/// It provides a mechanism to limit the number of times a secret can be exposed.
/// The behavior of the `Secret` type is customizable through various features, such as zeroization, cloning support, and debugging capabilities.
///
/// ## Type Parameters
/// - `T`: The underlying type of the secret.
/// - `MEC`: Maximum Exposure Count, a type-level unsigned integer, with `typenum::Unsigned` bound, indicating the maximum allowed exposures for the secret.
/// - `EC`: Exposure Count, a type-level unsigned integer, with `typenum::Unsigned` bound, representing the current exposure count of the secret.
/// It is limited by the Maximum Exposure Count, if `EC` is greater than `MEC`, the program cannot be compiled.
///
/// ## Features
/// - `zeroize` (optional): If enabled, the secret will be automatically zeroized (cleared) after reaching its maximum exposure count.
/// - `cloneable-secret` (optional): If enabled, the underlying type `T` must implement the `sosecrets_rs::traits::CloneableSecret` trait, allowing the secret to be cloned.
/// - `debug-secret` (optional): If enabled, the underlying type `T` must implement the `sosecrets_rs::traits::DebugSecret` trait, enabling debugging of the secret.
#[repr(transparent)]
pub struct Secret<
    #[cfg(feature = "zeroize")] T: Zeroize,
    #[cfg(not(feature = "zeroize"))] T,
    MEC: Unsigned + IsGreaterOrEqual<U0, Output = True> + Sub<U1>,
>(ManuallyDrop<T>, PhantomData<MEC>);

/// Type representing an exposed secret value. It holds an annotated (`'brand`) [invariant](https://doc.rust-lang.org/nomicon/subtyping.html#variance) lifetime.
pub struct ExposedSecret<'brand, T>(T, PhantomData<fn(&'brand ()) -> &'brand ()>);

impl<
        #[cfg(feature = "zeroize")] T: Zeroize,
        #[cfg(not(feature = "zeroize"))] T,
        MEC: Unsigned + IsGreaterOrEqual<U0, Output = True> + Sub<U1>,
    > Secret<T, MEC>
{
    /// Creates a new `Secret` instance with the specified value.
    ///
    /// # Parameters
    /// - `value`: The initial value to be stored in the secret.
    ///
    /// # Returns
    /// A new `Secret` instance initialized with the provided value.
    ///
    /// # Examples
    /// ```rust
    /// use sosecrets_rs::prelude::*;
    /// use typenum::U5;
    ///
    /// // Create a new secret with a maximum exposure count of 5
    /// let secret = Secret::<_, U5>::new("my_secret_value".to_string());
    /// ```
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self(ManuallyDrop::new(value), PhantomData)
    }

    /// Creates a new `Secret` instance by generating the value with a closure.
    ///
    /// # Parameters
    /// - `closure`: A closure that generates the initial value to be stored in the secret.
    ///
    /// # Returns
    /// A new `Secret` instance initialized with the value produced by the closure.
    ///
    /// # Examples
    /// ```rust
    /// use sosecrets_rs::prelude::*;
    /// use typenum::U3;
    ///
    /// // Create a new secret with a maximum exposure count of 3 using a closure
    /// let secret = Secret::<_, U3>::new_with(|| "generated_secret_value".to_string());
    /// ```
    #[inline(always)]
    pub fn new_with<ClosureType>(closure: ClosureType) -> Self
    where
        ClosureType: FnOnce() -> T,
    {
        Self(ManuallyDrop::new(closure()), PhantomData)
    }
}

impl<
        'max,
        #[cfg(feature = "zeroize")] T: Zeroize,
        #[cfg(not(feature = "zeroize"))] T,
        MEC: Unsigned + IsGreaterOrEqual<U0, Output = True> + Sub<U1>,
    > ExposeSecret<'max, &'max T, MEC> for Secret<T, MEC>
where
    Sub1<MEC>: Unsigned + IsGreaterOrEqual<U0, Output = True> + Sub<U1>,
{
    type Exposed<'brand> = ExposedSecret<'brand, &'brand T>
    where
        'max: 'brand;

    type Next = Secret<T, Sub1<MEC>>
    where
        Sub1<MEC>: Unsigned + IsGreaterOrEqual<U0, Output = True> + Sub<U1>;

    #[inline(always)]
    fn expose_secret<ReturnType, ClosureType>(
        mut self,
        scope: ClosureType,
    ) -> (Secret<T, Sub1<MEC>>, ReturnType)
    where
        Sub1<MEC>: Sub<U1> + Unsigned + IsGreaterOrEqual<U0, Output = True>,
        MEC: Unsigned + IsGreaterOrEqual<U0, Output = True> + Sub<U1>,
        for<'brand> ClosureType: FnOnce(ExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        let returned_value = scope(ExposedSecret(&self.0, PhantomData));
        // SAFETY: Since compile error prevents constructing a `Secret` with `EC` > `MEC`,
        // and it is not possible to call `expose_secret(...)`
        // when `Secret` is maximally exposed to access **private** `self.0` field,
        // therefore, this is safe.
        let inner = ManuallyDrop::new(unsafe { ManuallyDrop::take(&mut self.0) });
        forget(self);
        (Secret(inner, PhantomData), returned_value)
    }
}

impl<T> Deref for ExposedSecret<'_, &'_ T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        self.0
    }
}

impl<
        #[cfg(feature = "zeroize")] T: Zeroize,
        #[cfg(not(feature = "zeroize"))] T,
        MEC: Sub<U1> + Unsigned + IsGreaterOrEqual<U0, Output = True>,
    > Drop for Secret<T, MEC>
{
    #[inline(always)]
    fn drop(&mut self) {
        // SAFETY: Since compile error prevents constructing a `Secret` with `EC` > `MEC`,
        // and it is not possible to call `expose_secret(...)`
        // when `Secret` is maximally exposed to access **private** `self.0` field,
        // therefore, this is safe.
        let mut _inner = unsafe { ManuallyDrop::take(&mut self.0) };
        #[cfg(feature = "zeroize")]
        _inner.zeroize();
    }
}

#[cfg(feature = "cloneable-secret")]
impl<T, MEC> Clone for Secret<T, MEC>
where
    T: CloneableSecret,
    Sub1<MEC>: Sub<U1> + Unsigned + IsGreaterOrEqual<U0, Output = True>,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

#[cfg(feature = "debug-secret")]
impl<T, MEC> core::fmt::Debug for Secret<T, MEC>
where
    T: DebugSecret,
    Sub1<MEC>: Sub<U1> + Unsigned + IsGreaterOrEqual<U0, Output = True>,
{
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Secret<")?;
        T::debug_secret(f)?;
        f.write_str(">")
    }
}
