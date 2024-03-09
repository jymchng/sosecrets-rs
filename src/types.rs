use crate::traits::{ChooseMinimallyRepresentableUInt, __private};

impl __private::SealedTrait for NumericalZeroSizedType {}

impl ChooseMinimallyRepresentableUInt for NumericalZeroSizedType {
    type Output = NumericalZeroSizedType;
    type AtomicOutput = NumericalZeroSizedType;

    const ZERO: Self::Output = NumericalZeroSizedType {};
    const ONE: Self::Output = NumericalZeroSizedType {};

    fn cast_unsigned_to_self_type<T: typenum::Unsigned>(_: __private::SealedToken) -> Self::Output {
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
