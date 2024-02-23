use sosecrets_rs::{
    prelude::typenum::{U1, U2},
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
