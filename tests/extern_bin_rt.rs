#![no_std]
use sosecrets_rs::runtime::{
    secret::{ExposedSecret, RunTimeSecret},
    traits::ExposeSecret,
};

#[test]
fn test_bounds() {
    use core::marker::PhantomData;
    fn check_send<T: Send>() {}
    fn check_unpin<T: Unpin>() {}
    // This has to take a value, since the async fn's return type is unnameable.
    // fn check_send_sync_val<T: Send + Sync>(_t: T) {}
    // fn check_send_sync<T: Send + Sync>() {}
    check_unpin::<RunTimeSecret<i32, 2>>();
    check_send::<RunTimeSecret<i32, 2>>();
    check_unpin::<ExposedSecret<'_, PhantomData<fn(&()) -> &()>>>();
    check_send::<ExposedSecret<'_, PhantomData<fn(&()) -> &()>>>();

    // let secret = RunTimeSecret::<i32, 5>::new(69);
    // check_send_sync_val(secret);
    // let secret = RunTimeSecret::<i32, 5>::new(69);
    // check_send_sync_val(secret.expose_secret(|_| {}));

    // check_send_sync::<RunTimeSecret<i32, 2>>();
}

#[test]
fn test_expose_secret_runtime() {
    let secret_one = RunTimeSecret::<isize, 2>::new(69);

    let _ = secret_one.expose_secret(|exposed_secret| {
        assert_eq!(*exposed_secret, 69);
    });

    let _ = secret_one.expose_secret(|exposed_secret| {
        assert_eq!(*exposed_secret, 69);
    });
}

#[test]
#[should_panic = "`RunTimeSecret` has been exposed 3 times, more than what it is maximally allowed for: 2 times"]
fn test_expose_secret_runtime_should_panic() {
    let secret_one = RunTimeSecret::<isize, 2>::new(69);

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
