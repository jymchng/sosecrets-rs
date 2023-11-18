use sosecrets_rs::prelude::*;
mod common;

#[test]
fn test_expose_secret() {
    use common::{UseSecret, self};
    let secret = "MySecret";
    let new_secret: Secret<&str, 1, _> = Secret::new(secret);
    let (new_secret, returned_value) = new_secret.expose_secret(
        |exposed_secret| {
            let returned_value = UseSecret::new(*exposed_secret);
            (exposed_secret, returned_value)
        }
    );
    assert_eq!(returned_value.inner, secret);
}