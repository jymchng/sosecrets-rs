#![no_std]

mod common;

fn main() {
    use common::UseSecret;
    use sosecrets_rs::prelude::*;
    use sosecrets_rs::traits::ExposeSecret;
    use typenum::consts::U2;

    #[cfg(feature = "alloc")]
    extern crate std;

    #[cfg(feature = "alloc")]
    use std::{borrow::ToOwned, vec};

    // try similar with vec
    let secret_vec = vec!["MySecret".to_owned()];
    let new_secret: Secret<_, U2, _> = Secret::new(secret_vec);
    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, vec!["MySecret".to_owned()]);

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, vec!["MySecret".to_owned()]);

    let (_new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, vec!["MySecret".to_owned()]);
}
