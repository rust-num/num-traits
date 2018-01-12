#!/bin/bash

set -ex

echo Testing num-traits on rustc ${TRAVIS_RUST_VERSION}

# num-traits should build and test everywhere.
cargo build --verbose
cargo test --verbose

# It should build with minimal features too.
cargo build --no-default-features
cargo test --no-default-features

# num-traits with explicit std feature
cargo build --verbose --no-default-features --features="$feature"
cargo test --verbose --no-default-features --features="$feature"
