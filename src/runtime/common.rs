use crate::{runtime::traits::MinimallyRepresentableUInt, traits::__private};
use core::ops::Sub;

use typenum::{Exp, Unsigned, U1, U16, U2, U256, U32, U4294967296, U64, U65536, U8};

impl MinimallyRepresentableUInt for U8 {
    type Type = u8;
    type UIntMaxValueAsType = <U256 as Sub<U1>>::Output;
    const MIN: Self::Type = Self::Type::MIN;
    const ONE: Self::Type = 1;

    fn cast_unsigned_to_self_type<T: typenum::Unsigned>(_: __private::SealedToken) -> Self::Type {
        <T as Unsigned>::U8
    }
}

impl MinimallyRepresentableUInt for U16 {
    type Type = u16;
    type UIntMaxValueAsType = <U65536 as Sub<U1>>::Output;
    const MIN: Self::Type = Self::Type::MIN;
    const ONE: Self::Type = 1;

    fn cast_unsigned_to_self_type<T: typenum::Unsigned>(_: __private::SealedToken) -> Self::Type {
        <T as Unsigned>::U16
    }
}

impl MinimallyRepresentableUInt for U32 {
    type Type = u32;
    type UIntMaxValueAsType = <U4294967296 as Sub<U1>>::Output;
    const MIN: Self::Type = Self::Type::MIN;
    const ONE: Self::Type = 1;

    fn cast_unsigned_to_self_type<T: typenum::Unsigned>(_: __private::SealedToken) -> Self::Type {
        <T as Unsigned>::U32
    }
}

impl MinimallyRepresentableUInt for U64 {
    type Type = u64;
    type UIntMaxValueAsType = <Exp<U2, U64> as Sub<U1>>::Output;
    const MIN: Self::Type = Self::Type::MIN;
    const ONE: Self::Type = 1;

    fn cast_unsigned_to_self_type<T: typenum::Unsigned>(_: __private::SealedToken) -> Self::Type {
        <T as Unsigned>::U64
    }
}

#[cfg(target_pointer_width = "16")]
pub(crate) type DefaultMinimallyRepresentableUInt = typenum::U16;

#[cfg(target_pointer_width = "32")]
pub(crate) type DefaultMinimallyRepresentableUInt = typenum::U32;

#[cfg(target_pointer_width = "64")]
pub(crate) type DefaultMinimallyRepresentableUInt = typenum::U64;
