use crate::traits::ChooseMinimallyRepresentableUInt;

/// An error representing that the secret has been exposed more times than allowed.
#[derive(Debug)]
#[non_exhaustive]
pub enum ExposeSecretError<MEC: ChooseMinimallyRepresentableUInt> {
    ExposeMoreThanMaximallyAllow(ExposeMoreThanMaximallyAllowError<MEC>),
}

/// An error representing that the secret has been exposed more times than allowed.
#[derive(Debug)]
pub struct ExposeMoreThanMaximallyAllowError<MEC: ChooseMinimallyRepresentableUInt> {
    pub mec: <MEC as ChooseMinimallyRepresentableUInt>::Output,
    pub ec: <MEC as ChooseMinimallyRepresentableUInt>::Output,
}

impl<MEC: ChooseMinimallyRepresentableUInt> core::fmt::Display
    for ExposeMoreThanMaximallyAllowError<MEC>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "`Secret` is exposed more than what it is maximally allowed to; it is exposed for {} times and it is only allowed to be exposed for {} times", self.ec, self.mec)
    }
}

impl<MEC: ChooseMinimallyRepresentableUInt> core::fmt::Display for ExposeSecretError<MEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ExposeMoreThanMaximallyAllow(err) => err.fmt(f),
        }
    }
}
