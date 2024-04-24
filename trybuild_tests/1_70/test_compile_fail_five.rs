mod common;
fn main() {
    use sosecrets_rs::prelude::*;
    use typenum::consts::U2;

    struct A {
        inner: i32,
    }

    let new_secret: Secret<_, U2> = Secret::new(A { inner: 69 });

    let cloned_secret = new_secret.clone();
}
