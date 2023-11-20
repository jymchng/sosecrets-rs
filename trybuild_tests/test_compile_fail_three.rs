use sosecrets_rs::prelude::Secret;
use typenum::consts::{U5, U67};

fn main() {
    let secret = "MySecret".to_owned();
    let new_secret: Secret<_, U5, U67> = Secret::new(secret);
}
