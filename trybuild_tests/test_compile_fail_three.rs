use sosecrets_rs::prelude::*;
use typenum::consts::{U5, U67};
mod common;

fn main() {
    use common::UseSecret;

    let secret = "MySecret".to_owned();
    let new_secret: Secret<_, U5, U67> = Secret::new(secret);
}
