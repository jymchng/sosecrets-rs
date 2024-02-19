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
