# sosecrets-rs
<div align="center">
  <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/jymchng/sosecrets-rs/ci.yaml?label=build">
  <img alt="Crates.io" src="https://img.shields.io/crates/l/sosecrets-rs">
  <img alt="Crates.io" src="https://img.shields.io/crates/v/sosecrets-rs">
  <img alt="msrv" src="https://img.shields.io/badge/msrv-1.70.0-blue"/>
</div>

Aims to be a crate for Secrets Management in Rust.

# Minimum Supported Rust version

The crate currently requires Rust 1.70. I have no intent of increasing the
compiler version requirement of this crate beyond this. However, this is only
guaranteed within a given minor version number.

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
