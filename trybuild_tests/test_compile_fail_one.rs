use sosecrets_rs::prelude::*;
use typenum::consts::U5;
mod common;

fn main() {
    use common::UseSecret;

    let secret = "MySecret".to_owned();
    let new_secret: Secret<_, U5, _> = Secret::new(secret);
    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, "MySecret".to_owned());
}
