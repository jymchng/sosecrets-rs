use sosecrets_rs::{
    prelude::typenum::{U0, U1, U2},
    runtime::{
        secret::{RTExposedSecret, RTSecret},
        traits::RTExposeSecret,
    },
};

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
#[should_panic = "`RTSecret` has already been exposed 2 times, which is also the maximum number it is allowed to be exposed for."]
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

// #[test]
// fn test_expose_secret_runtime_cannot_return_secret() {

//     #[cfg(feature = "zeroize")]
//     use zeroize::Zeroize;

//     struct A {
//         inner: i32,
//     }

//     #[cfg(feature = "zeroize")]
//     impl Zeroize for A {
//         fn zeroize(&mut self) {
//             self.inner.zeroize()
//         }
//     }

//     let secret_one = RTSecret::<A, 2>::new(A {
//         inner: 69,
//     });

//     let _ = secret_one.expose_secret(|exposed_secret| {
//         exposed_secret
//     });

//     let _ = secret_one.expose_secret(|exposed_secret| {
//         *exposed_secret
//     });
// }
