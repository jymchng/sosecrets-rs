#[cfg(feature = "cloneable-secret")]
macro_rules! impl_cloneable_secret_for_numbers {
    ($($t:ty),*) => {
        $(
            impl $crate::traits::CloneableSecret for $t {}
        )*
    };
}

#[cfg(feature = "cloneable-secret")]
pub(crate) use impl_cloneable_secret_for_numbers;

#[cfg(feature = "debug-secret")]
macro_rules! impl_debug_secret_for_numbers {
    ($($t:ty),*) => {
        $(
            impl $crate::traits::DebugSecret for $t {}
        )*
    };
}

#[cfg(feature = "debug-secret")]
pub(crate) use impl_debug_secret_for_numbers;
