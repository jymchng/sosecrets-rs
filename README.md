# sosecrets-rs
<div align="center">
  <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/jymchng/sosecrets-rs/ci.yaml?style=for-the-badge" height="23">
  <a href="https://crates.io/crates/sosecrets-rs"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/sosecrets-rs?logo=rust&style=for-the-badge" height="23"></a>
  <a href="https://docs.rs/sosecrets-rs"><img alt="docs.rs" src="https://img.shields.io/crates/v/sosecrets-rs?color=blue&label=docs&style=for-the-badge" height="23"></a>
</div>

Secrets Management crate with

1. type level and compile-time guarantees and
2. each reference corresponds to each secret that can only be exposed or revealed under a lexical scope with an invariant lifetime

It is similar to the [`secrecy`](https://github.com/iqlusioninc/crates/tree/main/secrecy) crate but with type level and compile-time guarantees that the [`Secret<T, MEC, EC>`](prelude::Secret) value is not ’exposed’ more than `MEC` number of times and is only exposed under a well-defined lexical scope.

It makes use of the [`typenum`](https://github.com/paholg/typenum/tree/main) crate for all its compile-time guarantees.

## Features

- **Exposure Control:** Secret values can only be exposed a limited number of times, preventing unintentional information leaks. This is guaranteed at compile time. Secrets are exposed and available for use with an [invariant](https://doc.rust-lang.org/nomicon/subtyping.html#variance) lifetime, identifiable with a clear lexical scope.
- **Zeroization:** If configured with the "zeroize" feature, secrets are zeroized upon dropping them.
- **Cloneable Secrets:** With the "cloneable-secret" feature, `Secret` values can be cloned if the underlying type, `T`, implements the [`CloneableSecret`](traits::CloneableSecret) trait.
- **Debugging Secrets:** The "debug-secret" feature enables the debugging of `Secret` values if the underlying type, `T`, implements the [`DebugSecret`](traits::DebugSecret) trait.

## Usage Examples

## Compile Time Checks

```rust
use sosecrets_rs::{
  prelude::*,
  traits::ExposeSecret,
};
use typenum::U2;

// Define a secret with a maximum exposure count of 2
let secret = Secret::<_, U2>::new("my_secret_value".to_string());

// Expose the secret and perform some operations with the exposed value; secret has been exposed once: `EC` = 1, `MEC` = 2;
let (next_secret, exposed_value) = secret.expose_secret(|exposed_secret| {
  // `exposed_secret` is only 'available' from the next line -------
  assert_eq!(&*exposed_secret.as_str(), "my_secret_value"); //     ^
  // Perform operations with the exposed value                     |
  // ...                                                           v
  // to this line... -----------------------------------------------
});

// Expose the secret again and perform some operations with the exposed value; secret has been exposed twice: `EC` = 2, `MEC` = 2;
let (next_secret, exposed_value) = next_secret.expose_secret(|exposed_secret| {
  assert_eq!(&*exposed_secret.as_str(), "my_secret_value");
  // Perform operations with the exposed value
  // ...
});
```

**Try** to expose the secret again and perform some operations with the exposed value; secret has been exposed the third time: `EC` = 3, `MEC` = 2;
The following is uncompilable.
```compile_fail
let (next_secret, exposed_value) = next_secret.expose_secret(|exposed_secret| {
    assert_eq!(&*exposed_secret.as_str(), "my_secret_value");
    // Perform operations with the exposed value
    // ...
});
```

It is **impossible** to return the value (e.g. `exposed_secret` in the example above) passed into the closure, out of the closure.

The following is uncompilable.

```compile_fail
let (next_secret, exposed_value) = next_secret.expose_secret(|exposed_secret| {
    assert_eq!(&*exposed_secret.as_str(), "my_secret_value");
    // Perform operations with the exposed value
    // ...
    exposed_secret // impossible to return `exposed_secret` here
});
```

Note: If `T` is `Copy`, then the above will compile successfully and `expose_secret(...)` method will return a **copy** of exposed `T`.

## Runtime Checks

```rust
use sosecrets_rs::{
  prelude::*,
  // Note, for runtime checks, you have to use the `RTExposeSecret` trait instead.
  runtime::traits::RTExposeSecret,
};
use typenum::U2;

// Define a secret with a maximum exposure count of 2
let secret = RTSecret::<_, U2>::new("my_secret_value".to_string());

// Expose the secret and perform some operations with the exposed value; secret has been exposed once: `EC` = 1, `MEC` = 2;
let exposed_value = secret.expose_secret(|exposed_secret| {
  // `exposed_secret` is only 'available' from the next line -------
  assert_eq!(&*exposed_secret.as_str(), "my_secret_value"); //     ^
  // Perform operations with the exposed value                     |
  // ...                                                           v
  // to this line... -----------------------------------------------
});

// Expose the secret again and perform some operations with the exposed value; secret has been exposed twice: `EC` = 2, `MEC` = 2;
let exposed_value = secret.expose_secret(|exposed_secret| {
  assert_eq!(&*exposed_secret.as_str(), "my_secret_value");
  // Perform operations with the exposed value
  // ...
});
```

**Try** to expose the secret again and perform some operations with the exposed value; secret has been exposed the third time: `EC` = 3, `MEC` = 2;

`.expose_secret(...)` method will then `panic` with the message:

```bash
`RTSecret\` has already been exposed for 2 times, the maximum number it is allowed to be exposed for is 2 times."
```

```rust,should_panic
# use sosecrets_rs::{
#   prelude::*,
#   // Note, for runtime checks, you have to use the `RTExposeSecret` trait instead.
#   runtime::traits::RTExposeSecret,
# };
# use typenum::U2;
#
# // Define a secret with a maximum exposure count of 2
# let secret = RTSecret::<_, U2>::new("my_secret_value".to_string());
#
# // Expose the secret and perform some operations with the exposed value; secret has been exposed once: `EC` = 1, `MEC` = 2;
# let exposed_value = secret.expose_secret(|exposed_secret| {
#   // `exposed_secret` is only 'available' from the next line -------
#   assert_eq!(&*exposed_secret.as_str(), "my_secret_value"); //     ^
#   // Perform operations with the exposed value                     |
#   // ...                                                           v
#   // to this line... -----------------------------------------------
# });
#
# // Expose the secret again and perform some operations with the exposed value; secret has been exposed twice: `EC` = 2, `MEC` = 2;
# let exposed_value = secret.expose_secret(|exposed_secret| {
#   assert_eq!(&*exposed_secret.as_str(), "my_secret_value");
#   // Perform operations with the exposed value
#   // ...
# });
let exposed_value = secret.expose_secret(|exposed_secret| {
  assert_eq!(&*exposed_secret.as_str(), "my_secret_value");
  // Perform operations with the exposed value
  // ...
});
```

Note: You can use the non-panicking variant of the method `expose_secret(...)` which is named as `try_expose_secret(...)`.

`try_expose_secret(...)` returns a `Result::Err` if the exposure count is larger than what is maximally allowed.

It is **impossible** to return the value (e.g. `exposed_secret` in the example above) passed into the closure, out of the closure, **unless** `T` is `Copy`. The following is uncompilable.

```compile_fail
let exposed_value = secret.expose_secret(|exposed_secret| {
  assert_eq!(&*exposed_secret.as_str(), "my_secret_value");
  // Perform operations with the exposed value
  // ...
  exposed_secret // impossible to return `exposed_secret` here
});
```

## Substitute for the `secrecy` crate

You can use the [`SecrecySecret`](prelude::SecrecySecret) type as a substitute for the [`Secret<T>`](https://docs.rs/secrecy/0.8.0/secrecy/struct.Secret.html) in [`secrecy`](https://crates.io/crates/secrecy) crate.

```rust
use sosecrets_rs::{
  prelude::*,
  // Note, for runtime checks, you have to use the `RTExposeSecret` trait instead.
  runtime::traits::RTExposeSecret,
};

// Define a secret with NO maximum exposure count
let secret = SecrecySecret::new("my_secret_value".to_string());

// Expose the secret and perform some operations with the exposed value as many times as you like.
for _ in 0..=1_000_000 {
  let exposed_value = secret.expose_secret(|exposed_secret| {
  // `exposed_secret` is only 'available' from the next line -------
  assert_eq!(&*exposed_secret.as_str(), "my_secret_value"); //     ^
  // Perform operations with the exposed value                     |
  // ...                                                           v
  // to this line... -----------------------------------------------
  });
}
```

See more in the [examples](https://github.com/jymchng/sosecrets-rs/tree/master/examples/jwt) directory.

## Features Configuration

To enable features, you can include them in your `Cargo.toml`:

```toml
[dependencies]
sosecrets-rs = { version = "x.x.x", features = ["zeroize", "cloneable-secret", "debug-secret"] }
```

## Modules

- [`prelude`](prelude): Module for easily importing common items.
- [`runtime`](runtime): Module for [`RTSecret<T>`](prelude::RTSecret), [`SecrecySecret`](prelude::SecrecySecret) and [`RTExposeSecret`](runtime::traits::RTExposeSecret).

## Traits

- [`ExposeSecret`](traits::ExposeSecret): Trait for safely exposing secrets with a limited exposure count at compile time.
- [`RTExposeSecret`](runtime::traits::RTExposeSecret): Trait for safely exposing secrets with a limited exposure count at runtime time.
- [`CloneableSecret`](traits::CloneableSecret): Trait for cloneable secrets.
- [`DebugSecret`](traits::DebugSecret): Trait for debuggable secrets.

For example, if the feature `"cloneable-secret"` is enabled, then you can 'clone' the secret.

Example:
```rust
#[cfg(all(feature = "cloneable-secret", feature = "alloc"))]
// Need to enable feature = "alloc" because `String` requires feature = "alloc".
{
  use sosecrets_rs::{
      prelude::*,
      traits::{CloneableSecret, ExposeSecret},
  };
  use typenum::U2;

  // Define a secret with a maximum exposure count of 2
  let secret = Secret::<_, U2>::new("my_secret_value".to_string());

  // Clone the secret
  let secret2 = secret.clone();

  // Expose the secret and perform some operations with the exposed value; secret has been exposed once: `EC` = 1, `MEC` = 2;
  let (next_secret, exposed_value) = secret.expose_secret(move |exposed_secret| {
      // `exposed_secret` is only 'available' from the next line --------------------------^
      let (next_secret2, exposed_value2) = secret2.expose_secret(|exposed_secret2| { //    |
          assert_eq!(&*exposed_secret.as_str(), "my_secret_value"); //                     |
          assert_eq!(&*exposed_secret2.as_str(), "my_secret_value"); //                    |
          assert_eq!(&*exposed_secret2.as_str(), &*exposed_secret.as_str()); //            |
          // Perform operations with the exposed value                                      |
          // ...                                                                            |
          // to this line... ---------------------------------------------------------------v
      });
  });
}
```

# Minimum Supported Rust version

The crate currently requires Rust 1.70. I have no intent on increasing the compiler version requirement of this crate beyond this. However, this is only guaranteed within a given minor version number.

# Tests

Run

```bash
bash scripts/tests-all-features.sh
```

# License

Licensed under

- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the MIT license, without any additional terms or conditions.

# Credits

## [CAD97](https://github.com/CAD97)

* For rendering substantial help in the design and implementations of [`ExposeSecret`](traits::ExposeSecret) [[Rust Forum](https://users.rust-lang.org/t/making-a-value-of-a-type-undroppable-at-compile-time/102628/13?), [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=3c2e97e284c60c8a4067b77b6cfd72c7)] trait and its trait method, [`expose_secret(...)`](traits::ExposeSecret::expose_secret) [[Rust Forum](https://users.rust-lang.org/t/making-a-value-of-a-type-undroppable-at-compile-time/102628/6?), [Rust Playground](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=adce4708492654b6ad888f9a6b5bc5d0)].
* For teaching me the concept of [`invariant`](https://github.com/CAD97/generativity/blob/main/README.md) lifetime.

## [Eric Michael Sumner](https://orcid.org/0000-0002-6439-9757)

* For creating the macro `impl_choose_int!()` on [Rust Forum](https://users.rust-lang.org/t/making-a-type-level-type-function-with-typenum-crate/107008/3?). The macro helps to implement the trait [`ChooseMinimallyRepresentableUInt`](traits::ChooseMinimallyRepresentableUInt) for all type-level unsigned integers provided by the `typenum` crate that are representable from 1 bit to 64 bits at the type level.

## [Simon Farnsworth](https://users.rust-lang.org/u/farnz/summary)

* For providing advice on how to manage the optimizations done on [`RTSecret`](prelude::RTSecret) with regards to having the first field of the struct having different Rust's primitive unsigned integer types according to the type parameter `MEC` [[Link](https://users.rust-lang.org/t/rtsecret-t-std-cell-u8-is-the-same-size-as-rtsecret-t-std-cell-u16-why-and-how-to-optimize-such-that-former-is-smaller-than-latter/107396/20?)].
