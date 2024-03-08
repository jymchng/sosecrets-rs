fn main() {
    #[cfg(all(
        feature = "debug-secret",
        feature = "cloneable-secret",
        feature = "alloc"
    ))]
    use sosecrets_rs::runtime::traits::RTExposeSecret;
    use sosecrets_rs::{prelude::typenum::U5, runtime::secret::RTSecret};

    let vec_one = vec![1, 2, 3];

    let secret = RTSecret::<Vec<i32>, U5>::new(vec_one);

    let cloned_secret = secret.clone();
    let debug_secret = format!("{:?}", secret);

    assert_eq!("RTSecret<[REDACTED]>", debug_secret);

    let _ = cloned_secret.expose_secret(|exposed_secret| {
        assert_eq!(*exposed_secret, vec![1, 2, 3]);
    });
}
