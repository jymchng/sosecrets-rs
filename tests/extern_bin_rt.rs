use sosecrets_rs::{
    prelude::typenum::{U0, U1, U2},
    runtime::{traits::RTExposeSecret, RTExposedSecret, RTSecret},
};
mod common;

#[test]
fn test_bounds() {
    use core::marker::PhantomData;
    fn check_send<T: Send>() {}
    fn check_unpin<T: Unpin>() {}
    // This has to take a value, since the async fn's return type is unnameable.
    // fn check_send_sync_val<T: Send + Sync>(_t: T) {}
    // fn check_send_sync<T: Send + Sync>() {}
    check_unpin::<RTSecret<i32, U1>>();
    check_send::<RTSecret<i32, U1>>();
    check_unpin::<RTExposedSecret<'_, PhantomData<fn(&()) -> &()>>>();
    check_send::<RTExposedSecret<'_, PhantomData<fn(&()) -> &()>>>();

    // let secret = RTSecret::<i32, 5>::new(69);
    // check_send_sync_val(secret);
    // let secret = RTSecret::<i32, 5>::new(69);
    // check_send_sync_val(secret.expose_secret(|_| {}));

    // check_send_sync::<RTSecret<i32, 2>>();
}

// #[test]
// fn test_expose_secret_runtime_unchecked() {
//     let secret_one = RTSecret::<isize, U0>::new(69);

//     for _ in 0..=10000 {
//         let _ = secret_one.expose_secret(|exposed_secret| {
//             assert_eq!(*exposed_secret, 69);
//         });
//     }

//     use sosecrets_rs::runtime::secret::SecrecySecret;

//     let secret_two = SecrecySecret::new(69);

//     for _ in 0..=10000 {
//         let _ = secret_two.expose_secret(|exposed_secret| {
//             assert_eq!(*exposed_secret, 69);
//         });
//     }
// }

#[test]
fn test_expose_secret_runtime() {
    let secret_one = RTSecret::<isize, U2>::new(69);

    let _ = secret_one.expose_secret(|exposed_secret| {
        assert_eq!(*exposed_secret, 69);
    });

    let _ = secret_one.expose_secret(|exposed_secret| {
        assert_eq!(*exposed_secret, 69);
    });
}

#[test]
fn test_size_of_unchecked_secret() {
    use core::mem::size_of;

    assert_eq!(size_of::<RTSecret<isize, U0>>(), size_of::<isize>());
}

#[test]
#[should_panic = "`RTSecret` has already been exposed for 2 times, the maximum number it is allowed to be exposed for is 2 times."]
fn test_expose_secret_runtime_should_panic() {
    let secret_one = RTSecret::<isize, U2>::new(69);

    let _ = secret_one.expose_secret(|exposed_secret| {
        assert_eq!(*exposed_secret, 69);
    });

    let _ = secret_one.expose_secret(|exposed_secret| {
        assert_eq!(*exposed_secret, 69);
    });

    let _ = secret_one.expose_secret(|exposed_secret| {
        assert_eq!(*exposed_secret, 69);
    });
}

#[test]
fn test_min_uint_outcomes() {
    use core::ops::Sub;
    use sosecrets_rs::{
        prelude::typenum::{Add1, U1, U100, U1000, U365, U65536},
        traits::ChooseMinimallyRepresentableUInt,
    };

    type SubU1<K> = <K as Sub<U1>>::Output;

    assert_eq!(
        <U100 as ChooseMinimallyRepresentableUInt>::Output::MAX,
        u8::MAX
    );
    assert_eq!(
        <U1 as ChooseMinimallyRepresentableUInt>::Output::MAX,
        u8::MAX
    );
    assert_eq!(
        <U1000 as ChooseMinimallyRepresentableUInt>::Output::MAX,
        u16::MAX
    );
    assert_eq!(
        <U365 as ChooseMinimallyRepresentableUInt>::Output::MAX,
        u16::MAX
    );
    assert_eq!(
        <Add1<U65536> as ChooseMinimallyRepresentableUInt>::Output::MAX,
        u32::MAX
    );
    assert_eq!(
        <SubU1<U65536> as ChooseMinimallyRepresentableUInt>::Output::MAX,
        u16::MAX
    );

    let _: <U65536 as ChooseMinimallyRepresentableUInt>::Output = 65536;
    let _: <SubU1<U65536> as ChooseMinimallyRepresentableUInt>::Output = 65535;
    let _: <U1000 as ChooseMinimallyRepresentableUInt>::Output = 9999;
    let _: <U365 as ChooseMinimallyRepresentableUInt>::Output = 365 * 99;
}

#[test]
fn test_can_cross_unwind_boundaries_if_copy() {
    use core::panic::AssertUnwindSafe;

    extern crate std;
    use sosecrets_rs::{
        prelude::typenum::U2,
        runtime::{secret::RTSecret, traits::RTExposeSecret},
    };
    use std::panic::catch_unwind;

    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    #[derive(Copy, Clone)]
    struct A {
        inner: i32,
    }

    #[cfg(feature = "zeroize")]
    impl Zeroize for A {
        fn zeroize(&mut self) {
            self.inner.zeroize()
        }
    }

    let mut opt_a: Option<A> = Option::<A>::None;

    let secret_one = RTSecret::<A, U2>::new(A { inner: 69 });

    let _ = catch_unwind(AssertUnwindSafe(|| {
        secret_one.expose_secret(|exposed_secret| {
            opt_a.replace(*exposed_secret);
            panic!();
        });
    }));
    assert_eq!(opt_a.unwrap().inner, 69);
}

#[test]
#[should_panic = "`RTSecret` has already been exposed for 1 times, the maximum number it is allowed to be exposed for is 1 times."]
fn test_unwind_can_catch_panic_but_secret_will_continue_to_panic() {
    use core::panic::AssertUnwindSafe;

    extern crate std;
    use sosecrets_rs::{
        prelude::typenum::U1,
        runtime::{secret::RTSecret, traits::RTExposeSecret},
    };
    use std::panic::catch_unwind;

    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    #[derive(Copy, Clone)]
    struct A {
        _inner: i32,
    }

    #[cfg(feature = "zeroize")]
    impl Zeroize for A {
        fn zeroize(&mut self) {
            self._inner.zeroize()
        }
    }

    let secret_one = RTSecret::<A, U1>::new(A { _inner: 69 });

    let _ = secret_one.expose_secret(|_| {});

    let _ = catch_unwind(AssertUnwindSafe(|| {
        secret_one.expose_secret(|_| {});
    }));

    let _ = catch_unwind(AssertUnwindSafe(|| {
        secret_one.expose_secret(|_| {});
    }));

    let _ = catch_unwind(AssertUnwindSafe(|| {
        secret_one.expose_secret(|_| {});
    }));

    let _ = catch_unwind(AssertUnwindSafe(|| {
        secret_one.expose_secret(|_| {});
    }));

    let _ = secret_one.expose_secret(|exposed_secret| *exposed_secret);
}

#[test]
fn test_unwind_can_catch_panic_indefinitely() {
    use core::panic::AssertUnwindSafe;

    extern crate std;
    use sosecrets_rs::{
        prelude::typenum::U1,
        runtime::{secret::RTSecret, traits::RTExposeSecret},
    };
    use std::panic::catch_unwind;

    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    #[derive(Copy, Clone)]
    struct A {
        _inner: i32,
    }

    #[cfg(feature = "zeroize")]
    impl Zeroize for A {
        fn zeroize(&mut self) {
            self._inner.zeroize()
        }
    }

    let secret_one = RTSecret::<A, U1>::new(A { _inner: 69 });

    let _ = secret_one.expose_secret(|_| {});

    let _ = catch_unwind(AssertUnwindSafe(|| {
        secret_one.expose_secret(|_| {});
    }));

    let _ = catch_unwind(AssertUnwindSafe(|| {
        secret_one.expose_secret(|_| {});
    }));

    let _ = catch_unwind(AssertUnwindSafe(|| {
        secret_one.expose_secret(|_| {});
    }));

    let _ = catch_unwind(AssertUnwindSafe(|| {
        secret_one.expose_secret(|_| {});
    }));
}

#[test]
#[cfg(feature = "debug-secret")]
fn test_debug_secret_one() {
    use core::fmt::Write;
    use sosecrets_rs::{prelude::typenum::U5, traits::DebugSecret};
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

    let mut cmp = common::Comparator::new("RTSecret<[REDACTED]>");

    let new_secret: RTSecret<A, U5> = RTSecret::new(a.clone());
    let _ = write!(&mut cmp, "{:?}", new_secret);
    assert!(cmp.is_valid());
}

#[cfg(feature = "cloneable-secret")]
#[cfg(feature = "alloc")]
#[test]
fn test_secret_with_vec_and_clone() {
    use crate::common::UseSecret;

    let secret_vec = vec!["MySecret".to_string()];
    let new_secret: RTSecret<_, U2> = RTSecret::new(secret_vec);
    let returned_value = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, vec!["MySecret".to_owned()]);

    let cloned_secret = new_secret.clone();
    let returned_value = cloned_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, vec!["MySecret".to_owned()]);
}

#[test]
fn test_expose_secret_with_wrapper() {
    use crate::common::UseSecret;
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
    let new_secret: RTSecret<_, U50> = RTSecret::new(secret);
    let returned_value = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, SecretString("MySecret".to_owned()));

    let returned_value = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, SecretString("MySecret".to_owned()));

    let returned_value = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, SecretString("MySecret".to_owned()));

    let returned_value = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, SecretString("MySecret".to_owned()));

    let returned_value = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(returned_value.inner, SecretString("MySecret".to_owned()));
}

#[cfg(feature = "cloneable-secret")]
#[test]
fn test_clone_1() {
    use crate::common::UseSecret;
    let new_secret: RTSecret<_, U2> = RTSecret::new(69);

    let returned_value = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new(*exposed_secret);
        returned_value
    });
    assert_eq!(69, returned_value.inner);

    let cloned_secret = new_secret.clone();
    let returned_value = cloned_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new(*exposed_secret);
        returned_value
    });
    assert_eq!(69, returned_value.inner);
}

#[test]
fn test_with_new() {
    use crate::common::UseSecret;
    use sosecrets_rs::prelude::typenum::U5;
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

    let new_secret: RTSecret<SecretString, U5> = RTSecret::new_with(|| {
        SecretString(
            env::var("CARGO_TARGET_DIR")
                .unwrap_or("MySecret".to_string())
                .to_string(),
        )
    });

    let returned_value = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(
        returned_value.inner,
        SecretString(env::var("CARGO_TARGET_DIR").unwrap_or("MySecret".to_string()))
    );

    let returned_value = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(
        returned_value.inner,
        SecretString(env::var("CARGO_TARGET_DIR").unwrap_or("MySecret".to_string()))
    );

    let returned_value = new_secret.expose_secret(|exposed_secret| {
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
    use crate::common::UseSecret;
    use sosecrets_rs::prelude::typenum::U5;
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

    let new_secret: RTSecret<SecretString, U5> = RTSecret::new_with(|| {
        SecretString(
            env::var("CARGO_TARGET_DIR")
                .unwrap_or("MySecret".to_string())
                .to_string(),
        )
    });

    let returned_value = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(
        returned_value.inner,
        SecretString(env::var("CARGO_TARGET_DIR").unwrap_or("MySecret".to_string()))
    );

    let returned_value = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(
        returned_value.inner,
        SecretString(env::var("CARGO_TARGET_DIR").unwrap_or("MySecret".to_string()))
    );

    let returned_value = new_secret.expose_secret(|exposed_secret| {
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
    use crate::common::UseSecret;
    use sosecrets_rs::prelude::typenum::U5;
    use std::env;
    use zeroize::Zeroize;

    #[derive(Clone, Debug, PartialEq)]
    struct SecretString(String);

    impl Zeroize for SecretString {
        fn zeroize(&mut self) {
            self.0.zeroize();
        }
    }

    let new_secret: RTSecret<SecretString, U5> = RTSecret::new_with(|| {
        SecretString(
            env::var("CARGO_TARGET_DIR")
                .unwrap_or("MySecret".to_string())
                .to_string(),
        )
    });

    let returned_value = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(
        returned_value.inner,
        SecretString(env::var("CARGO_TARGET_DIR").unwrap_or("MySecret".to_string()))
    );

    let returned_value = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_owned());
        returned_value
    });
    assert_eq!(
        returned_value.inner,
        SecretString(env::var("CARGO_TARGET_DIR").unwrap_or("MySecret".to_string()))
    );

    let returned_value = new_secret.expose_secret(|exposed_secret| {
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

    let new_secret = RTSecret::<i32, U2>::new_with(|| 69);
    let new_secret_two = RTSecret::<String, U2>::new_with(|| "69".to_owned());

    scope(|s| {
        s.spawn(move || {
            let returned_value = new_secret.expose_secret(|exposed_secret| *exposed_secret);
            assert_eq!(69, returned_value);
            let returned_value = new_secret.expose_secret(|exposed_secret| *exposed_secret);
            assert_eq!(69, returned_value);
        });
        s.spawn(move || {
            let returned_value =
                new_secret_two.expose_secret(|exposed_secret| exposed_secret.to_owned());
            assert_eq!("69".to_owned(), returned_value);
            let returned_value =
                new_secret_two.expose_secret(|exposed_secret| exposed_secret.to_owned());
            assert_eq!("69".to_owned(), returned_value);
        });
    });
}

#[test]
fn test_scoped_threads_the_other_way_round() {
    use std::thread::scope;

    let new_secret = RTSecret::<i32, U2>::new_with(|| 69);

    let _ = new_secret.expose_secret(|exposed_secret| {
        scope(|s| {
            let scope_handler = s.spawn(move || *exposed_secret);
            let result = scope_handler.join();
            assert_eq!(result.unwrap(), 69);
        });
    });
}

#[test]
fn test_never_exposed_fully_but_dropped() {
    use core::sync::atomic::{AtomicUsize, Ordering};
    use sosecrets_rs::{prelude::typenum::U5, runtime::traits::RTExposeSecret};
    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    #[cfg(feature = "zeroize")]
    impl Zeroize for DetectDrop {
        fn zeroize(&mut self) {}
    }

    static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

    struct DetectDrop;

    impl Drop for DetectDrop {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, Ordering::Relaxed);
        }
    }

    {
        let secret = RTSecret::<DetectDrop, U5>::new(DetectDrop);

        {
            let _ = secret.expose_secret(|_exposed_secret| {});
            assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0usize);
            let _ = secret.expose_secret(|_exposed_secret| {});
            assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0usize);
            let _ = secret.expose_secret(|_exposed_secret| {});
            assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0usize);
        }
    }

    assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 1usize);
}

#[test]
fn test_exposed_fully_but_dropped() {
    use core::sync::atomic::{AtomicUsize, Ordering};
    use sosecrets_rs::runtime::traits::RTExposeSecret;
    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    #[cfg(feature = "zeroize")]
    impl Zeroize for DetectDrop {
        fn zeroize(&mut self) {}
    }

    static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);
    struct DetectDrop;

    impl Drop for DetectDrop {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, Ordering::Relaxed);
        }
    }

    {
        let secret = RTSecret::<DetectDrop, U2>::new(DetectDrop);

        {
            let _ = secret.expose_secret(|_exposed_secret| {});
            assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0usize);
            let _ = secret.expose_secret(|_exposed_secret| {});
            assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0usize);
        }
    }

    assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 1usize);
}
