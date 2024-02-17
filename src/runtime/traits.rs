use core::{
    fmt::{Debug, Display},
    ops::AddAssign,
};

pub use crate::runtime::error;
use num_traits::AsPrimitive;
use typenum::Unsigned;

pub trait RTExposeSecret<'secret, T, SIZE: MinimallyRepresentableUInt> {
    type Exposed<'brand>
    where
        'secret: 'brand;

    fn expose_secret<ReturnType, ClosureType>(&self, scope: ClosureType) -> ReturnType
    where
        for<'brand> ClosureType: FnOnce(Self::Exposed<'brand>) -> ReturnType;

    fn try_expose_secret<ReturnType, ClosureType>(
        &self,
        scope: ClosureType,
    ) -> Result<ReturnType, error::ExposeSecretError<SIZE>>
    where
        for<'brand> ClosureType: FnOnce(Self::Exposed<'brand>) -> ReturnType;
}

pub(crate) trait MinimallyRepresentableUInt: Unsigned {
    type Type: AddAssign + AsPrimitive<usize> + Debug + Display;
    type UIntMaxValueAsType;
    const MIN: Self::Type;
    const ONE: Self::Type;
}
