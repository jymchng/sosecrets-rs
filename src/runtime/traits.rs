pub use crate::runtime::error;

use crate::traits::ChooseMinimallyRepresentableUInt;
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
