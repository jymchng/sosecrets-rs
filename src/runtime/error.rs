use crate::runtime::traits::MinimallyRepresentableUInt;

#[derive(Debug)]
#[non_exhaustive]
pub enum ExposeSecretError<SIZE: MinimallyRepresentableUInt> {
    ExposeMoreThanMaximallyAllow(ExposeMoreThanMaximallyAllowError<SIZE>),
}

#[derive(Debug)]
pub struct ExposeMoreThanMaximallyAllowError<SIZE: MinimallyRepresentableUInt> {
    pub mec: usize,
    pub ec: SIZE::Type,
}

impl<SIZE: MinimallyRepresentableUInt> core::fmt::Display
    for ExposeMoreThanMaximallyAllowError<SIZE>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "`Secret` is exposed more than what it is maximally allowed to; it is exposed for {} times and it is only allowed to be exposed for {} times", self.ec, self.mec)
    }
}

impl<SIZE: MinimallyRepresentableUInt> core::fmt::Display for ExposeSecretError<SIZE> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ExposeMoreThanMaximallyAllow(err) => err.fmt(f),
        }
    }
}
