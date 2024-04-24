#!/bin/bash

cargo eval --help &> /dev/null

if [ $? -ne 0 ]; then
    echo "cargo eval is NOT installed, installing it now"
    cargo install --force cargo-eval
    echo "installed cargo eval"
else
    echo "cargo eval is installed"
fi

rust_version=$(rustc --version | cut -d ' ' -f 2)

if [ "$rust_version" != "1.70.0" ]; then
    echo "Rust version is not 1.70.0, now setting it to 1.70.0; the previous version is $rust_version"
    rustup override set 1.70
fi

bash scripts/tests-all-features.sh

rustup override unset

echo "Rustup toolchain resetted to default; COMMAND RAN: rustup override unset"

echo "========== RUST TOOLCHAIN =========="
rustup show
echo "===================================="
