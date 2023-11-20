use sosecrets_rs::prelude::*;
use typenum::consts::U5;
mod common;

#[test]
fn test_expose_secret_extern() {
    use common::{self, UseSecret};
    use sosecrets_rs::traits::ExposeSecret;

    let secret = "MySecret".to_owned();
    let new_secret: Secret<_, U5, _> = Secret::new(secret);
    let (_new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, "MySecret".to_owned());
}

// #[test]
// fn test_expose_secret_in_a_loop() {
//     use common::{self, UseSecret};
//     use sosecrets_rs::traits::ExposeSecret;

//     let secret = "MySecret".to_owned();
//     let new_secret: Secret<_, U50, _> = Secret::new(secret);

//     for _i in 0..=49 {
//         let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
//             let returned_value = UseSecret::new((*exposed_secret).to_owned());
//             returned_value
//         });
//         assert_eq!(returned_value.inner, "MySecret".to_owned());
//     }
// }
