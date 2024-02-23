fn main() {
    use sosecrets_rs::{
        prelude::typenum::U2,
        runtime::{secret::RTSecret, traits::RTExposeSecret},
    };

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

    let secret_one = RTSecret::<A, U2>::new(A { inner: 69 });

    let _ = secret_one.expose_secret(|exposed_secret| exposed_secret);

    let _ = secret_one.expose_secret(|exposed_secret| *exposed_secret);
}
