fn main() {
    use core::panic::AssertUnwindSafe;
    use sosecrets_rs::{
        prelude::{typenum::U2, Secret},
        traits::ExposeSecret,
    };
    use std::panic::catch_unwind;
    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    #[cfg(feature = "zeroize")]
    impl<T: Zeroize> Zeroize for A<T> {
        fn zeroize(&mut self) {
            self.0.zeroize()
        }
    }

    #[derive(Debug)]
    struct A<#[cfg(feature = "zeroize")] T: Zeroize, #[cfg(not(feature = "zeroize"))] T>(T);

    let new_secret = Secret::<A<i32>, U2>::new_with(|| A(69));

    let mut opt: Option<A<i32>> = None;
    let _ = catch_unwind(AssertUnwindSafe(|| {
        new_secret.expose_secret(|exposed_secret| {
            opt = Some(*exposed_secret);
            panic!();
        });
    }));
    assert_eq!(opt.unwrap().0, 69);
}
