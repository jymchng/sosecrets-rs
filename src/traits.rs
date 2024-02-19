use crate::macros::impl_choose_int;
use core::{
    cmp::PartialOrd,
    fmt::{Debug, Display},
    ops::{Add, AddAssign},
};
use typenum::{IsLessOrEqual, Sum, True, Unsigned, U1};

/// A trait for safely exposing secrets with a limited exposure count.
///
/// The `ExposeSecret` trait provides a mechanism to progressively expose a secret
/// value in a controlled manner, with an invariant lifetime and compile-time guarantees.
/// It allows for limiting the exposure of a secret to a maximum count (`MEC`).
/// The exposure count (`EC`) is tracked at compile time to ensure that it does not exceed the specified maximum count.
///
/// # Type Parameters
/// - `'max`: A lifetime parameter indicating the lifetime of the value of the type that implements this trait.
/// - `T`: The type of the secret being exposed.
/// - `MEC`: A type-level unsigned integer (with `typenum::Unsigned` trait bound) representing the maximum exposure count.
/// - `EC`: A type-level unsigned integer (with `typenum::Unsigned` trait bound) representing the current exposure count.
pub trait ExposeSecret<'max, T, MEC: Unsigned, EC: Unsigned>: Sized {
    /// A wrapper type representing the exposed secret. It is associated with a lifetime `'brand`, indicating the lifetime of the wrapper type, which is strictly a subtype of `'max`,
    type Exposed<'brand>
    where
        'max: 'brand;

    /// The `Secret<T, _, _>` with an incremented count (i.e. `EC`) after exposing the secret.
    /// It is a new value of a type which implements the same trait, namely, `ExposeSecret` with an incremented exposure count, i.e. the new `EC` = previous `EC` + `1`.
    type Next: ExposeSecret<'max, T, MEC, Sum<EC, U1>>
    where
        EC: Add<U1> + Unsigned + IsLessOrEqual<MEC, Output = True>,
        Sum<EC, U1>: Unsigned + IsLessOrEqual<MEC, Output = True> + Add<U1>;

    /// Exposes the secret and returns the `Secret<T, _, _>` with an incremented count (i.e. `EC`), along with the result of a provided closure.
    ///
    /// # Parameters
    /// - `self`.
    /// - `scope`: A closure (of the type given by the type parameter `ClosureType`) that takes the exposed secret, of type `Exposed<'brand>` and returns a result, of type `ReturnType`.
    ///
    /// Returns `(Self::Next, ReturnType)`
    fn expose_secret<ReturnType, ClosureType>(self, scope: ClosureType) -> (Self::Next, ReturnType)
    where
        for<'brand> ClosureType: FnOnce(Self::Exposed<'brand>) -> ReturnType,
        EC: Add<U1> + IsLessOrEqual<MEC, Output = True>,
        Sum<EC, U1>: Unsigned + Add<U1> + IsLessOrEqual<MEC, Output = True>;
}

#[cfg(feature = "cloneable-secret")]
pub use self::cloneable_secret::CloneableSecret;

#[cfg(feature = "debug-secret")]
pub use self::debug_secret::DebugSecret;

#[cfg(feature = "cloneable-secret")]
mod cloneable_secret {
    //! Traits and implementations related to cloneable secrets.

    use core::clone::Clone;

    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    /// A trait for cloneable secrets.
    ///
    /// This trait extends the standard `Clone` trait for types that represent secrets,
    /// allowing them to be cloned.
    #[cfg(feature = "zeroize")]
    pub trait CloneableSecret: Clone + Zeroize {}

    /// A trait for cloneable secrets.
    ///
    /// This trait extends the standard `Clone` trait for types that represent secrets,
    /// allowing them to be cloned.
    #[cfg(not(feature = "zeroize"))]
    pub trait CloneableSecret: Clone {}

    impl<
            #[cfg(feature = "zeroize")] T: Clone + Zeroize,
            #[cfg(not(feature = "zeroize"))] T: Clone,
            const N: usize,
        > CloneableSecret for [T; N]
    {
    }

    #[cfg(feature = "alloc")]
    use alloc::{string::String, vec::Vec};

    #[cfg(feature = "alloc")]
    impl CloneableSecret for String {}

    #[cfg(feature = "alloc")]
    impl<
            #[cfg(feature = "zeroize")] T: Clone + Zeroize,
            #[cfg(not(feature = "zeroize"))] T: Clone,
        > CloneableSecret for Vec<T>
    {
    }

    crate::macros::impl_cloneable_secret_for_numbers!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[cfg(feature = "debug-secret")]
mod debug_secret {
    use core::fmt::Debug;

    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    /// A trait for debuggable secrets.
    ///
    /// This trait extends the standard `Debug` trait for types that represent secrets,
    /// allowing them to be formatted for debugging purposes.
    #[cfg(feature = "zeroize")]
    pub trait DebugSecret: Debug + Zeroize {
        /// Formats the secret as "`[REDACTED]`".
        fn debug_secret(f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            f.write_str("[REDACTED]")
        }
    }

    /// A trait for debuggable secrets.
    ///
    /// This trait extends the standard `Debug` trait for types that represent secrets,
    /// allowing them to be formatted for debugging purposes.
    #[cfg(not(feature = "zeroize"))]
    pub trait DebugSecret: Debug {
        /// Formats the secret as "`[REDACTED]`".
        fn debug_secret(f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            f.write_str("[REDACTED]")
        }
    }

    impl<
            #[cfg(feature = "zeroize")] T: Debug + Zeroize,
            #[cfg(not(feature = "zeroize"))] T: Debug,
            const N: usize,
        > DebugSecret for [T; N]
    {
    }

    #[cfg(feature = "alloc")]
    use alloc::{string::String, vec::Vec};

    #[cfg(feature = "alloc")]
    impl DebugSecret for String {}

    #[cfg(feature = "alloc")]
    impl<
            #[cfg(feature = "zeroize")] T: Debug + Zeroize,
            #[cfg(not(feature = "zeroize"))] T: Debug,
        > DebugSecret for Vec<T>
    {
    }

    crate::macros::impl_debug_secret_for_numbers!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

pub(crate) mod __private {

    pub struct SealedToken {}
    pub trait SealedTrait {}
}

pub trait ChooseMinimallyRepresentableUInt: __private::SealedTrait {
    type Output: AddAssign + PartialOrd + Debug + Display + Copy;
}

impl_choose_int! {
    B00 => u8;
    B01 => u8;
    B02 => u8;
    B03 => u8;
    B04 => u8;
    B05 => u8;
    B06 => u8;
    B07 => u8;

    B10 => u16;
    B11 => u16;
    B12 => u16;
    B13 => u16;
    B14 => u16;
    B15 => u16;
    B16 => u16;
    B17 => u16;

    B20 => u32;
    B21 => u32;
    B22 => u32;
    B23 => u32;
    B24 => u32;
    B25 => u32;
    B26 => u32;
    B27 => u32;

    B30 => u32;
    B31 => u32;
    B32 => u32;
    B33 => u32;
    B34 => u32;
    B35 => u32;
    B36 => u32;
    B37 => u32;

    B40 => u64;
    B41 => u64;
    B42 => u64;
    B43 => u64;
    B44 => u64;
    B45 => u64;
    B46 => u64;
    B47 => u64;

    B50 => u64;
    B51 => u64;
    B52 => u64;
    B53 => u64;
    B54 => u64;
    B55 => u64;
    B56 => u64;
    B57 => u64;

    B60 => u64;
    B61 => u64;
    B62 => u64;
    B63 => u64;
    B64 => u64;
    B65 => u64;
    B66 => u64;
    B67 => u64;

    B70 => u64;
    B71 => u64;
    B72 => u64;
    B73 => u64;
    B74 => u64;
    B75 => u64;
    B76 => u64;
    B77 => u64;
}
