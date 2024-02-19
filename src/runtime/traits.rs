use core::{
    cmp::PartialOrd,
    fmt::{Debug, Display},
    ops::AddAssign,
};

pub use crate::runtime::error;

use crate::traits::__private;
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

pub trait MinimallyRepresentableUInt: Unsigned {
    // indeed, `u8`, `u16`, `u32` and `u64` all satisfy these bounds
    type Type: AddAssign + PartialOrd + Debug + Display + Copy;
    type UIntMaxValueAsType;
    const MIN: Self::Type;
    const ONE: Self::Type;

    fn cast_unsigned_to_self_type<T: Unsigned>(_: __private::SealedToken) -> Self::Type;
}
