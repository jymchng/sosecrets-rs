mod common;

fn main() {
    use common::UseSecret;
    use sosecrets_rs::prelude::*;
    use sosecrets_rs::traits::ExposeSecret;
    use typenum::consts::U2;

    // try similar with vec
    let secret_vec = vec!["MySecret".to_string()];
    let new_secret: Secret<_, U2, _> = Secret::new(secret_vec);
    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, vec!["MySecret".to_owned()]);

    let cloned_secret = new_secret.clone();
    let (cloned_secret, returned_value) = cloned_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, vec!["MySecret".to_owned()]);

    // Cloned `Secret` over exposed here
    let (_cloned_secret, returned_value) = cloned_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, vec!["MySecret".to_owned()]);
}
