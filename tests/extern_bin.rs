use sosecrets_rs::{prelude::*, traits::ExposeSecret};
use typenum::consts::{U2, U5};
mod common;
use common::UseSecret;
#[cfg(feature = "debug-secret")]
use core::fmt::Write;

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
    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    #[derive(Clone, Debug, PartialEq)]
    struct SecretString(String);

    #[cfg(feature = "zeroize")]
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
    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    #[derive(Clone, Debug, PartialEq)]
    struct SecretString(String);

    #[cfg(feature = "zeroize")]
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
    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    #[derive(Clone, Debug, PartialEq)]
    struct SecretString(String);

    #[cfg(feature = "zeroize")]
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
fn test_scoped_threads() {
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
fn test_scoped_threads_the_other_way_round() {
    use std::thread::scope;

    let new_secret = Secret::<i32, U2>::new_with(|| 69);

    let (_new_secret, _) = new_secret.expose_secret(|exposed_secret| {
        scope(|s| {
            let scope_handler = s.spawn(move || *exposed_secret);
            let result = scope_handler.join();
            assert_eq!(result.unwrap(), 69);
        });
    });
}

// #[test]
// fn test_unscoped_concurrency_the_other_way_round() {
//     use std::thread::spawn;

//     let new_secret = Secret::<i32, U2>::new_with(|| 69);

//     let (_new_secret, _) = new_secret.expose_secret(|exposed_secret| {
//         let join_handler = spawn(move || assert_eq!(69, *exposed_secret)); // this thread has 'static lifetime
//         join_handler.join();
//     });
// }

#[test]
fn test_panic() {
    use core::panic::AssertUnwindSafe;
    use std::panic::catch_unwind;

    let new_secret = Secret::<i32, U2>::new_with(|| 69);

    let mut opt: Option<i32> = None;
    let _ = catch_unwind(AssertUnwindSafe(|| {
        new_secret.expose_secret(|exposed_secret| {
            let _ = opt.insert(*exposed_secret);
            panic!();
        });
    }));
    assert_eq!(opt.unwrap(), 69);
}

#[cfg(feature = "debug-secret")]
struct Comparator<'a> {
    valid: bool,
    to_compare: &'a str,
}

#[cfg(feature = "debug-secret")]
impl<'a> Comparator<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            valid: true,
            to_compare: s,
        }
    }

    fn is_valid(self) -> bool {
        self.valid && self.to_compare.is_empty()
    }
}

#[cfg(feature = "debug-secret")]
impl<'a> Write for Comparator<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        if s.eq(self.to_compare) {
            self.valid = self.valid && true;
            self.to_compare = "";
            return Ok(());
        }

        if self.to_compare.starts_with(s) && self.to_compare.len() >= s.len() {
            self.to_compare = &self.to_compare[s.len()..];
        } else {
            self.valid = false
        }
        Ok(())
    }
}

#[test]
#[cfg(feature = "debug-secret")]
fn test_debug_secret_one() {
    use sosecrets_rs::traits::DebugSecret;
    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    #[derive(Debug, Clone)]
    struct A {
        _inner: i32,
    }

    #[cfg(feature = "zeroize")]
    impl Zeroize for A {
        fn zeroize(&mut self) {
            self._inner.zeroize()
        }
    }

    impl DebugSecret for A {}

    let a = A { _inner: 69 };

    let mut cmp = Comparator::new("Secret<[REDACTED]>");

    let new_secret: Secret<A, U5> = Secret::new(a.clone());
    let _ = write!(&mut cmp, "{:?}", new_secret);
    assert!(cmp.is_valid());
}
