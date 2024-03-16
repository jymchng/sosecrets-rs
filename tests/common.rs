#[derive(Debug)]
pub struct UseSecret<T> {
    pub inner: T,
}
impl<T> UseSecret<T> {
    pub fn new(value: T) -> Self {
        Self { inner: value }
    }
}

#[cfg(feature = "debug-secret")]
pub struct Comparator<'a> {
    valid: bool,
    to_compare: &'a str,
}

#[cfg(feature = "debug-secret")]
impl<'a> Comparator<'a> {
    pub const fn new(s: &'a str) -> Self {
        Self {
            valid: true,
            to_compare: s,
        }
    }

    pub const fn is_valid(self) -> bool {
        self.valid && self.to_compare.is_empty()
    }
}

#[cfg(feature = "debug-secret")]
impl<'a> core::fmt::Write for Comparator<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        if s.eq(self.to_compare) {
            self.valid = self.valid && true;
            self.to_compare = "";
            return Ok(());
        }

        if self.to_compare.starts_with(s) && self.to_compare.len() >= s.len() {
            self.to_compare = &self.to_compare[s.len()..];
        } else {
            self.valid = false
        }
        Ok(())
    }
}
