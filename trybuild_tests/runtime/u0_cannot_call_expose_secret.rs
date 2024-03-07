fn main() {
    use sosecrets_rs::{
        prelude::typenum::{U0, U100},
        runtime::{
            secret::{RTSecret, SecrecySecret},
            traits::RTExposeSecret,
        },
    };

    let secret_one = RTSecret::<isize, U0>::new(69);

    // Not Ok
    for _ in 0..=10000 {
        let _ = secret_one.expose_secret(|exposed_secret| {
            assert_eq!(*exposed_secret, 69);
        });
    }

    let secret_two = RTSecret::<isize, U100>::new(69);

    for _ in 0..=99 {
        let _ = secret_two.expose_secret(|exposed_secret| {
            assert_eq!(*exposed_secret, 69);
        });
    }

    let secret_three = SecrecySecret::<i32>::new(71);

    // Not Ok
    for _ in 0..=10000 {
        let _ = secret_three.expose_secret(|exposed_secret| {
            assert_eq!(*exposed_secret, 71);
        });
    }
}
