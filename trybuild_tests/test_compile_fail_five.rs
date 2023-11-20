mod common;
fn main() {
    use common::UseSecret;
    use sosecrets_rs::prelude::*;
    use sosecrets_rs::traits::{CloneableSecret, ExposeSecret};
    use typenum::consts::U2;

    let new_secret: Secret<_, U2> = Secret::new(69);

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new(*exposed_secret);
        returned_value
    });
    assert_eq!(69, returned_value.inner);

    let cloned_secret = new_secret.clone_secret();
    let (_cloned_secret, returned_value) = cloned_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new(*exposed_secret);
        returned_value
    });
    assert_eq!(69, returned_value.inner);
    let (_cloned_secret, returned_value) = cloned_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new(*exposed_secret);
        returned_value
    });
    assert_eq!(69, returned_value.inner);
}
