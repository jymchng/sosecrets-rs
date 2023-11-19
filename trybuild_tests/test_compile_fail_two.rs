use sosecrets_rs::{prelude::*, traits::ExposeSecret};
use typenum::consts::U2;
mod common;

fn main() {
    use common::UseSecret;

    let new_secret: Secret<String, U2> = Secret::new("mySecret".to_string());

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_string());
        (exposed_secret, returned_value)
    });
    assert_eq!("mySecret", &returned_value.inner);

    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_string());
        (exposed_secret, returned_value)
    });
    assert_eq!("mySecret", &returned_value.inner);

    // Compilation fails
    let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
        let returned_value = UseSecret::new((*exposed_secret).to_string());
        (exposed_secret, returned_value)
    });
    assert_eq!("mySecret", returned_value.inner);

    //     error[E0271]: type mismatch resolving `<UInt<UInt<UTerm, B1>, B0> as IsLess>::Output == B1`
    // --> trybuild_tests/test_compile_fail_two.rs:22:51
    // |
    // 22 |     let (new_secret, returned_value) = new_secret.expose_secret(|exposed_secret| {
    // |                                                   ^^^^^^^^^^^^^ expected `B1`, found `B0`
    // |
    // note: required by a bound in `expose_secret`
    // --> src/traits.rs
    // |
    // |     fn expose_secret<ReturnType>(
    // |        ------------- required by a bound in this associated function
    // ...
    // |         EC: IsLess<MEC, Output = True>;
    // |                         ^^^^^^^^^^^^^ required by this bound in `ExposeSecret::expose_secret`

}
