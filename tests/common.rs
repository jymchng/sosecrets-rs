#[derive(Debug)]
pub struct UseSecret<T> {
    pub inner: T,
}
impl<T> UseSecret<T> {
    pub fn new(value: T) -> Self {
        Self { inner: value }
    }
}
