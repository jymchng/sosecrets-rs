fn main() {
    use sosecrets_rs::{
        prelude::{typenum::U2, Secret},
        traits::ExposeSecret,
    };

    #[derive(Debug, Clone)]
    struct GlobalA {
        _inner: i32,
    }

    let not_static_a: GlobalA = GlobalA { _inner: 70 };

    let secret_two: Secret<&GlobalA, U2> = Secret::new(&not_static_a);

    let (_, exposed_secret_two) = secret_two.expose_secret(|exposed_secret| *exposed_secret);

    assert_eq!(exposed_secret_two._inner, 70);
    assert_eq!(not_static_a._inner, 70);
}
