use sosecrets_rs::{prelude::*, traits::ExposeSecret};
use typenum::consts::{U2, U5};
mod common;
use common::UseSecret;

#[test]
fn test_expose_secret_extern() {
    let secret = "MySecret".to_owned();
    let new_secret: Secret<_, U5, _> = Secret::new(secret);
    let (_new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, "MySecret".to_owned());
}

#[cfg(feature = "cloneable-secret")]
#[test]
fn test_secret_with_vec_and_clone() {
    let secret_vec = vec!["MySecret".to_string()];
    let new_secret: Secret<_, U2, _> = Secret::new(secret_vec);
    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, vec!["MySecret".to_owned()]);

    let cloned_secret = new_secret.clone();
    let (_cloned_secret, returned_value) = cloned_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, vec!["MySecret".to_owned()]);
}

#[test]
fn test_expose_secret_with_wrapper() {
    use typenum::U50;
    use zeroize::Zeroize;

    #[derive(Clone, Debug, PartialEq)]
    struct SecretString(String);

    impl Zeroize for SecretString {
        fn zeroize(&mut self) {
            self.0.zeroize();
        }
    }

    let secret = SecretString("MySecret".to_owned());
    let new_secret: Secret<_, U50, _> = Secret::new(secret);
    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, SecretString("MySecret".to_owned()));

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, SecretString("MySecret".to_owned()));

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, SecretString("MySecret".to_owned()));

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, SecretString("MySecret".to_owned()));

    let (_new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, SecretString("MySecret".to_owned()));
}

// #[test]
// fn test_expose_secret_in_a_loop() {
//     use common::{self, UseSecret};
//     use sosecrets_rs::traits::ExposeSecret;
//     use typenum::U50;

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

#[test]
fn test_expose_secret() {
    let new_secret: Secret<String, U2> = Secret::new("mySecret".to_owned());

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).clone());
        returned_value
    });
    assert_eq!("mySecret", &returned_value.inner);

    let (_new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_string());
        returned_value
    });
    assert_eq!("mySecret", &returned_value.inner);
}

#[test]
fn test_expose_secret_2() {
    let new_secret: Secret<_, U2> = Secret::new(69);

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new(*exposed_secret);
        returned_value
    });
    assert_eq!(69, returned_value.inner);

    let (_new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new(*exposed_secret);
        returned_value
    });
    assert_eq!(69, returned_value.inner);
}

#[cfg(feature = "cloneable-secret")]
#[test]
fn test_clone_1() {
    let new_secret: Secret<_, U2> = Secret::new(69);

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new(*exposed_secret);
        returned_value
    });
    assert_eq!(69, returned_value.inner);

    let cloned_secret = new_secret.clone();
    let (_cloned_secret, returned_value) = cloned_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new(*exposed_secret);
        returned_value
    });
    assert_eq!(69, returned_value.inner);
}

#[test]
fn test_destruct_secret_1() {
    use sosecrets_rs::traits::SecretIntoInner;

    let new_secret: Secret<_, U2> = Secret::new(69);

    let got_out_inner_value = new_secret.into_inner();
    assert_eq!(got_out_inner_value, 69);
}
