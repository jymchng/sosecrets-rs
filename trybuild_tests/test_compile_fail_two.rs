use sosecrets_rs::{prelude::*, traits::ExposeSecret};
use typenum::consts::U2;
mod common;

fn main() {
    use common::UseSecret;

    let new_secret: Secret<String, U2> = Secret::new("mySecret".to_string());

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_string());
        returned_value
    });
    assert_eq!("mySecret", &returned_value.inner);

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_string());
        returned_value
    });
    assert_eq!("mySecret", &returned_value.inner);

    // Compilation fails
    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_string());
        returned_value
    });
    assert_eq!("mySecret", returned_value.inner);
}
