#!/bin/bash

set -ex

echo Testing num-traits on rustc ${TRAVIS_RUST_VERSION}

# num-traits should build and test everywhere.
cargo build --verbose
cargo test --verbose

# test `no_std`
cargo build --verbose --no-default-features
cargo test --verbose --no-default-features

if [[ "$TRAVIS_RUST_VERSION" =~ ^(nightly|beta|stable)$ ]]; then
    # test `i128`
    cargo build --verbose --features=i128
    cargo test --verbose --features=i128

    # test with std and libm (libm build fails on Rust 1.26 and earlier)
    cargo build --verbose --features "libm"
    cargo test --verbose --features "libm"

    # test `no_std` with libm (libm build fails on Rust 1.26 and earlier)
    cargo build --verbose --no-default-features --features "libm"
    cargo test --verbose --no-default-features --features "libm"
fi
