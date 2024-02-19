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

macro_rules! impl_choose_int {
    // Entry point
    ($($arg:ident => $out:ty;)*) => {
        impl_choose_int! {
            @prev_args ();
            @prev_num $crate::prelude::typenum::UTerm;
            @rest_args ($($arg,)*);
            @rest_out ($($out;)*);
        }
    };

    // Implement one
    (
        @prev_args ($($prev_args:ident,)*);
        @prev_num $prev_num:ty;
        @rest_args ($arg:ident, $($rest_args:ident,)*);
        @rest_out ($out:ty; $($rest_out:ty;)*);
    )
    => {
        impl<$($prev_args,)* $arg> $crate::traits::__private::SealedTrait for $crate::prelude::typenum::uint::UInt<$prev_num, $arg> {}

        impl<$($prev_args,)* $arg> $crate::traits::ChooseMinimallyRepresentableUInt for $crate::prelude::typenum::uint::UInt<$prev_num, $arg> {
            type Output = $out;
        }

        impl_choose_int!{
            @prev_args ($($prev_args,)* $arg,);
            @prev_num $crate::prelude::typenum::uint::UInt<$prev_num, $arg>;
            @rest_args ($($rest_args,)*);
            @rest_out ($($rest_out;)*);
        }
    };

    // Base case; stop iteration
    (
        @prev_args ($($prev_args:ident,)*);
        @prev_num $prev_num:ty;
        @rest_args ();
        @rest_out ();
    ) => {};
}

pub(crate) use impl_choose_int;
