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
#[cfg(feature = "alloc")]
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
fn test_with_new() {
    use std::env;
    use zeroize::Zeroize;

    #[derive(Clone, Debug, PartialEq)]
    struct SecretString(String);

    impl Zeroize for SecretString {
        fn zeroize(&mut self) {
            self.0.zeroize();
        }
    }

    let new_secret: Secret<SecretString, U5> = Secret::new_with(|| {
        SecretString(
            env::var("CARGO_TARGET_DIR")
                .unwrap_or("MySecret".to_string())
                .to_string(),
        )
    });

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(
        returned_value.inner,
        SecretString(env::var("CARGO_TARGET_DIR").unwrap_or("MySecret".to_string()))
    );

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(
        returned_value.inner,
        SecretString(env::var("CARGO_TARGET_DIR").unwrap_or("MySecret".to_string()))
    );

    let (_new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(
        returned_value.inner,
        SecretString(env::var("CARGO_TARGET_DIR").unwrap_or("MySecret".to_string()))
    );
}

#[cfg(feature = "cloneable-secret")]
#[test]
fn test_with_new_cloneable_secret() {
    use std::env;
    use zeroize::Zeroize;

    #[derive(Clone, Debug, PartialEq)]
    struct SecretString(String);

    impl Zeroize for SecretString {
        fn zeroize(&mut self) {
            self.0.zeroize();
        }
    }

    let new_secret: Secret<SecretString, U5> = Secret::new_with(|| {
        SecretString(
            env::var("CARGO_TARGET_DIR")
                .unwrap_or("MySecret".to_string())
                .to_string(),
        )
    });

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(
        returned_value.inner,
        SecretString(env::var("CARGO_TARGET_DIR").unwrap_or("MySecret".to_string()))
    );

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(
        returned_value.inner,
        SecretString(env::var("CARGO_TARGET_DIR").unwrap_or("MySecret".to_string()))
    );

    let (_new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(
        returned_value.inner,
        SecretString(env::var("CARGO_TARGET_DIR").unwrap_or("MySecret".to_string()))
    );
}

#[cfg(feature = "alloc")]
#[test]
fn test_with_new_alloc() {
    use std::env;
    use zeroize::Zeroize;

    #[derive(Clone, Debug, PartialEq)]
    struct SecretString(String);

    impl Zeroize for SecretString {
        fn zeroize(&mut self) {
            self.0.zeroize();
        }
    }

    let new_secret: Secret<SecretString, U5> = Secret::new_with(|| {
        SecretString(
            env::var("CARGO_TARGET_DIR")
                .unwrap_or("MySecret".to_string())
                .to_string(),
        )
    });

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(
        returned_value.inner,
        SecretString(env::var("CARGO_TARGET_DIR").unwrap_or("MySecret".to_string()))
    );

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(
        returned_value.inner,
        SecretString(env::var("CARGO_TARGET_DIR").unwrap_or("MySecret".to_string()))
    );

    let (_new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(
        returned_value.inner,
        SecretString(env::var("CARGO_TARGET_DIR").unwrap_or("MySecret".to_string()))
    );
}

#[test]
fn test_concurrency() {
    use std::thread::scope;

    let new_secret = Secret::<i32, U2>::new_with(|| 69);
    let new_secret_two = Secret::<String, U2>::new_with(|| "69".to_owned());

    scope(|s| {
        s.spawn(move || {
            let (new_secret, returned_value) =
                new_secret.expose_secret(|exposed_secret| *exposed_secret);
            assert_eq!(69, returned_value);
            let (_new_secret, returned_value) =
                new_secret.expose_secret(|exposed_secret| *exposed_secret);
            assert_eq!(69, returned_value);
        });
        s.spawn(move || {
            let (new_secret_two, returned_value) =
                new_secret_two.expose_secret(|exposed_secret| exposed_secret.to_owned());
            assert_eq!("69".to_owned(), returned_value);
            let (_new_secret_two, returned_value) =
                new_secret_two.expose_secret(|exposed_secret| exposed_secret.to_owned());
            assert_eq!("69".to_owned(), returned_value);
        });
    });
}

#[test]
fn test_concurrency_the_other_way_round() {
    use std::thread::scope;

    let new_secret = Secret::<i32, U2>::new_with(|| 69);

    let (_new_secret, _) = new_secret.expose_secret(|exposed_secret| {
        scope(|s| {
            let scope_handler = s.spawn(move || exposed_secret.clone());
            let result = scope_handler.join();
            assert_eq!(result.unwrap(), 69);
        });
    });
}
