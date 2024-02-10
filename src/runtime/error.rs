#[derive(Debug)]
#[non_exhaustive]
pub enum ExposeSecretError {
    ExposeMoreThanMaximallyAllow(ExposeMoreThanMaximallyAllowError),
}

#[derive(Debug)]
pub struct ExposeMoreThanMaximallyAllowError {
    pub mec: usize,
    pub ec: usize,
}

impl core::fmt::Display for ExposeMoreThanMaximallyAllowError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "`Secret` is exposed more than what it is maximally allowed to; it is exposed for {} times and it is only allowed to be exposed for {} times", self.ec, self.mec)
    }
}

impl core::fmt::Display for ExposeSecretError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ExposeMoreThanMaximallyAllow(err) => err.fmt(f),
        }
    }
}
