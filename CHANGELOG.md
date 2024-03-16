# CHANGELOG

## 17 March 2024
<center><h1>V0.2.0 IS RELEASED</h1></center>

1. `RTSecret<T, MEC>` is now available. `RTSecret<T, MEC>` is the runtime version of `Secret<T, MEC, EC>`.
2. Substitute of the `Secret<S>` in `secrecy` crate is available as `SecrecySecret<T>` albeit with a different function signature of the method `.expose_secret(...)`.
3. Following optimizations are done:

   a. Given the type parameter `MEC` as an unsigned type-level integer, the second field of `RTSecret<T, MEC>` will be one of the following Rust's primitive unsigned integer types, `u8`, `u16`, `u32` and `u64`, depending on the projected runtime value of `MEC`.

   For example, if `MEC` is `typenum::consts::U69`, then the second field of `RTSecret<T, MEC>` will be of type `u8`. This optimization is done because if `MEC` at runtime is projected to a value, e.g. 69, and the exposure counter, which is the second field of the type, will never exceed the `MEC`'s projected runtime value, then it only needs to be represented by an unsigned integer type (e.g. `u8`) that `MEC`'s projected runtime value (e.g. 69) is minimally representable by (since 69 < `u8::MAX` = 255).

   b. `SecrecySecret<T>` is the same size as `T`.
