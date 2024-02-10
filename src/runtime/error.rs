#[non_exhaustive]
pub enum ExposeSecretError {
    ExposeMoreThanMaximallyAllow,
}

#[derive(Debug)]
struct ExposeMoreThanMaximallyAllow {
    mec: usize,
    ec: usize,
}

impl core::fmt::Display for ExposeMoreThanMaximallyAllow {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "`Secret` is exposed more than what it is maximally allowed to; it is exposed for {} times and it is only allowed to be exposed for {} times", self.ec, self.mec)
    }
}
