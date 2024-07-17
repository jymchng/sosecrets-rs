# CHANGELOG

## 18 July 2024

1. Added a logo and many badges to README.

## 25 April 2024

1. Added directory `1_70` under `trybuild_tests` directory to ensure the tests pass and make sense for Minimum Supported Rust Version 1.70.
2. Wrote a build script `scripts/build-ver-tests.rs.rs` to copy the existing tests into the `1_70` directory.
3. Made sure that this build script is only ran during testing phase by trigger it using `cargo eval scripts/build-ver-tests.rs`.
4. GitHub Actions updated to run `scripts/tests-all-features-1.70.sh`, which is a bash script that wraps `scripts/tests-all-features.sh` to do testing on Rust version 1.70.

## 04 April 2024
<center><h1>V0.2.1 IS RELEASED</h1></center>

1. Minor fixes to `README.md`.

## 17 March 2024
<center><h1>V0.2.0 IS RELEASED</h1></center>

1. `RTSecret<T, MEC>` is now available. `RTSecret<T, MEC>` is the runtime version of `Secret<T, MEC, EC>`.
2. Substitute of the `Secret<S>` in `secrecy` crate is available as `SecrecySecret<T>` albeit with a different function signature of the method `.expose_secret(...)`.
3. Following optimizations are done:

   a. Given the type parameter `MEC` as an unsigned type-level integer, the second field of `RTSecret<T, MEC>` will be one of the following Rust's primitive unsigned integer types, `u8`, `u16`, `u32` and `u64`, depending on the projected runtime value of `MEC`.

   For example, if `MEC` is `typenum::consts::U69`, then the second field of `RTSecret<T, MEC>` will be of type `u8`. This optimization is done because if `MEC` at runtime is projected to a value, e.g. 69, and the exposure counter, which is the second field of the type, will never exceed the `MEC`'s projected runtime value, then it only needs to be represented by an unsigned integer type (e.g. `u8`) that `MEC`'s projected runtime value (e.g. 69) is minimally representable by (since 69 < `u8::MAX` = 255).

   b. `SecrecySecret<T>` is the same size as `T`.
