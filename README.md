# sosecrets-rs
<div align="center">
  <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/jymchng/sosecrets-rs/ci.yaml?label=build&&style=for-the-badge" height="23">
  <a href="https://crates.io/crates/sosecrets-rs"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/sosecrets-rs?logo=rust&style=for-the-badge" height="23"></a>
  <a href="https://docs.rs/sosecrets-rs"><img alt="docs.rs" src="https://img.shields.io/crates/v/sosecrets-rs?color=blue&label=docs&style=for-the-badge" height="23"></a>
</div>

Secrets Management crate with

1. type level and compile-time guarantees and
2. each reference corresponds to each secret that can only be exposed or revealed under a lexical scope with an invariant lifetime

It is similar to the [`secrecy`](https://github.com/iqlusioninc/crates/tree/main/secrecy) crate but with type level and compile-time guarantees that the `Secret<T, MEC, EC>` value is not ’exposed’ more than `MEC` number of times and is only exposed under a well-defined lexical scope.

It makes use of the [`typenum`](https://github.com/paholg/typenum/tree/main) crate for all its compile-time guarantees.

## Features

- **Exposure Control:** Secret values can only be exposed a limited number of times, preventing unintentional information leaks. This is guaranteed at compile time. Secrets are exposed and available for use with an [invariant](https://doc.rust-lang.org/nomicon/subtyping.html#variance) lifetime, identifiable with a clear lexical scope.
- **Zeroization:** If configured with the "zeroize" feature, secrets are zeroized upon dropping them.
- **Cloneable Secrets:** With the "cloneable-secret" feature, `Secret` values can be cloned if the underlying type, `T`, implements the `CloneableSecret` trait.
- **Debugging Secrets:** The "debug-secret" feature enables the debugging of `Secret` values if the underlying type, `T`, implements the `DebugSecret` trait.

## Usage Example

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

// Expose the secret again and perform some operations with the exposed value; secret has been exposed once: `EC` = 2, `MEC` = 2;
let (next_secret, exposed_value) = next_secret.expose_secret(|exposed_secret| {
    assert_eq!(&*exposed_secret.as_str(), "my_secret_value");
    // Perform operations with the exposed value
    // ...
});
```

**Try** to expose the secret again and perform some operations with the exposed value; secret has been exposed once: `EC` = 3, `MEC` = 2;
The following is uncompilable.
```compile_fail
let (next_secret, exposed_value) = next_secret.expose_secret(|exposed_secret| {
    assert_eq!(&*exposed_secret.as_str(), "my_secret_value");
    // Perform operations with the exposed value
    // ...
});
```

It is **impossible** to return the value (e.g. `exposed_secret` in the example above) passed into the closure, out of the closure. The following is uncompilable.

```compile_fail
let (next_secret, exposed_value) = next_secret.expose_secret(|exposed_secret| {
    assert_eq!(&*exposed_secret.as_str(), "my_secret_value");
    // Perform operations with the exposed value
    // ...
    exposed_secret // impossible to return `exposed_secret` here
});
```

See more in the [examples](https://github.com/jymchng/sosecrets-rs/tree/master/examples/jwt) directory.

## Features Configuration

To enable features, you can include them in your `Cargo.toml`:

```toml
[dependencies]
sosecrets-rs = { version = "x.x.x", features = ["zeroize", "cloneable-secret", "debug-secret"] }
```

## Modules

- `prelude`: Module for easily importing common items.

## Traits

- `ExposeSecret`: Trait for safely exposing secrets with a limited exposure count.
- `CloneableSecret`: Trait for cloneable secrets.
- `DebugSecret`: Trait for debuggable secrets.

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

* For rendering substantial help in the design and implementations of `ExposeSecret` [[Rust Forum](https://users.rust-lang.org/t/making-a-value-of-a-type-undroppable-at-compile-time/102628/13?), [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=3c2e97e284c60c8a4067b77b6cfd72c7)] trait and its trait method, `expose_secret(...)` [[Rust Forum](https://users.rust-lang.org/t/making-a-value-of-a-type-undroppable-at-compile-time/102628/6?), [Rust Playground](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=adce4708492654b6ad888f9a6b5bc5d0)].
* For teaching me the concept of [`invariant`](https://github.com/CAD97/generativity/blob/main/README.md) lifetime.
