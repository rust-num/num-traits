#!/bin/bash

set -ex

echo Testing num-traits on rustc ${TRAVIS_RUST_VERSION}

# num-traits should build and test everywhere.
cargo build --verbose
cargo test --verbose

# test with std and libm
cargo build --verbose --features "libm"
cargo test --verbose --features "libm"

# test `no_std`
cargo build --verbose --no-default-features
cargo test --verbose --no-default-features

# test `no_std` with libm
cargo build --verbose --no-default-features --features "libm"
cargo test --verbose --no-default-features --features "libm"


# test `i128`
if [[ "$TRAVIS_RUST_VERSION" =~ ^(nightly|beta|stable)$ ]]; then
    cargo build --verbose --features=i128
    cargo test --verbose --features=i128
fi
