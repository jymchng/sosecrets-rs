mod common;
fn main() {
    use sosecrets_rs::prelude::*;
    use sosecrets_rs::traits::CloneableSecret;
    use typenum::consts::U2;
    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    struct A {
        inner: i32,
    }
    #[cfg(feature = "zeroize")]
    impl Zeroize for A {
        fn zeroize(&mut self) {
            self.inner.zeroize()
        }
    }
    let new_secret: Secret<_, U2> = Secret::new(A { inner: 69 });
    impl Clone for A {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl CloneableSecret for A {}

    let _cloned_secret = new_secret.clone();
}
