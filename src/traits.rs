use core::ops::Add;
use typenum::{IsLessOrEqual, Sum, True, Unsigned, U1};

pub trait ExposeSecret<'max, T, MEC: Unsigned, EC: Unsigned>: Sized {
    type Exposed<'brand>
    where
        'max: 'brand;

    type Next: ExposeSecret<'max, T, MEC, Sum<EC, U1>>
    where
        EC: Add<U1> + Unsigned + IsLessOrEqual<MEC, Output = True>,
        Sum<EC, U1>: Unsigned + IsLessOrEqual<MEC, Output = True> + Add<U1>;

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

    use core::clone::Clone;

    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    #[cfg(feature = "zeroize")]
    pub trait CloneableSecret: Clone + Zeroize {}

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

    crate::impl_cloneable_secret_for_numbers!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[cfg(feature = "debug-secret")]
mod debug_secret {
    use core::fmt::Debug;

    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    #[cfg(feature = "zeroize")]
    pub trait DebugSecret: Debug + Zeroize {
        fn debug_secret(f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            f.write_str("[REDACTED]")
        }
    }

    #[cfg(not(feature = "zeroize"))]
    pub trait DebugSecret: Debug {
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

    crate::impl_debug_secret_for_numbers!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}
