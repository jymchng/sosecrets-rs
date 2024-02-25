use core::{
    cmp::PartialOrd,
    fmt::{Debug, Display},
    ops::AddAssign,
};

pub use crate::runtime::error;

use crate::traits::{ChooseMinimallyRepresentableUInt, __private};
use typenum::{IsGreater, True, Unsigned, U0};

pub trait RTExposeSecretUnchecked<'secret, T> {
    type Exposed<'brand>
    where
        'secret: 'brand;

    fn expose_secret_unchecked<ReturnType, ClosureType>(&self, scope: ClosureType) -> ReturnType
    where
        for<'brand> ClosureType: FnOnce(Self::Exposed<'brand>) -> ReturnType;
}

pub trait RTExposeSecret<
    'secret,
    T,
    MEC: ChooseMinimallyRepresentableUInt + Unsigned + IsGreater<U0, Output = True>,
>
{
    type Exposed<'brand>
    where
        'secret: 'brand;

    fn expose_secret<ReturnType, ClosureType>(&self, scope: ClosureType) -> ReturnType
    where
        MEC: IsGreater<U0, Output = True>,
        for<'brand> ClosureType: FnOnce(Self::Exposed<'brand>) -> ReturnType;

    fn try_expose_secret<ReturnType, ClosureType>(
        &self,
        scope: ClosureType,
    ) -> Result<ReturnType, error::ExposeSecretError<MEC>>
    where
        MEC: IsGreater<U0, Output = True>,
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
