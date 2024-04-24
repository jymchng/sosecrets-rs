mod common;
fn main() {
    use sosecrets_rs::{prelude::*, traits::ExposeSecret};
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

    let (new_secret, _) = new_secret.expose_secret(|exposed_secret| exposed_secret);
    let (_, _) = new_secret.expose_secret(|exposed_secret| *exposed_secret);
}
