fn main() {
    use core::panic::AssertUnwindSafe;

    extern crate std;
    use sosecrets_rs::{
        prelude::typenum::U2,
        runtime::{secret::RTSecret, traits::RTExposeSecret},
    };
    use std::panic::catch_unwind;

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

    let mut opt_a: Option<A> = Option::<A>::None;

    let secret_one = RTSecret::<A, U2>::new(A { inner: 69 });

    let _ = catch_unwind(AssertUnwindSafe(|| {
        secret_one.expose_secret(|exposed_secret| {
            opt_a.replace(*exposed_secret);
            panic!();
        });
    }));
    assert_eq!(opt_a.unwrap().inner, 69);
}
