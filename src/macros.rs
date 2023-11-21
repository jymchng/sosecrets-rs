#[macro_export]
macro_rules! impl_cloneable_secret_for_numbers {
    ($($t:ty),*) => {
        $(
            impl $crate::traits::CloneableSecret for $t {}
        )*
    };
}
