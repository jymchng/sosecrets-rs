mod common;

fn main() {
    use common::UseSecret;
    use sosecrets_rs::prelude::*;
    use sosecrets_rs::traits::{ExposeSecret, SecretIntoInner};
    use typenum::consts::U2;

    let new_secret: Secret<_, U2> = Secret::new(69);

    let got_out_inner_value = new_secret.into_inner();
    assert_eq!(got_out_inner_value, 69);

    let (_new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new(*exposed_secret);
        returned_value
    });
    assert_eq!(69, returned_value.inner);
}
