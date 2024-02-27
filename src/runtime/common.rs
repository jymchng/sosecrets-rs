use crate::traits::{
    AsAtomic, ChooseMinimallyRepresentableUInt,
    __private::{SealedToken, SealedTrait},
};
use typenum::{Unsigned, U0};

pub type Unchecked = U0;

impl SealedTrait for Unchecked {}

impl ChooseMinimallyRepresentableUInt for Unchecked {
    type Output = NumericalZeroSizedType;
    type AtomicOutput = NumericalZeroSizedType;

    const MIN: Self::Output = NumericalZeroSizedType {};
    const ONE: Self::Output = NumericalZeroSizedType {};

    fn cast_unsigned_to_self_type<T: Unsigned>(_: SealedToken) -> Self::Output {
        NumericalZeroSizedType {}
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct NumericalZeroSizedType {}

impl core::ops::AddAssign<Self> for NumericalZeroSizedType {
    fn add_assign(&mut self, _other: Self) {}
}

impl core::ops::Add<Self> for NumericalZeroSizedType {
    type Output = Self;

    fn add(self, _other: Self) -> Self::Output {
        NumericalZeroSizedType {}
    }
}

impl core::fmt::Display for NumericalZeroSizedType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "NumericalZeroSizedType")
    }
}

impl AsAtomic for u8 {
    type Output = core::sync::atomic::AtomicU8;
}

impl AsAtomic for u16 {
    type Output = core::sync::atomic::AtomicU16;
}

impl AsAtomic for u32 {
    type Output = core::sync::atomic::AtomicU32;
}

impl AsAtomic for u64 {
    type Output = core::sync::atomic::AtomicU64;
}
