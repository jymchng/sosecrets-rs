#![no_std]

mod common;
fn main() {
    use sosecrets_rs::{prelude::*, traits::ExposeSecret};
    use typenum::consts::U2;

    // use std for allocator + panic handler
    extern crate std;

    pub struct UseSecret<T> {
        pub inner: T,
    }
    impl<T> UseSecret<T> {
        pub fn new(value: T) -> Self {
            Self { inner: value }
        }
    }

    let new_secret: Secret<_, U2> = Secret::new(69);

    let (_new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new(*exposed_secret);
        returned_value
    });
    assert_eq!(69, returned_value.inner);
}
