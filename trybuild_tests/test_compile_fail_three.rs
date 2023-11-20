use sosecrets_rs::prelude::Secret;
use typenum::consts::{U5, U67};

fn main() {
    let secret = "MySecret".to_owned();
    // Tests EC as type argument must be lesser than MEC on the LHS
    let new_secret: Secret<String, U5, U67> = Secret::new(secret);
}
