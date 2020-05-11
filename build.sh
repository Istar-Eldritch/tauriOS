#!/usr/bin/env bash

set -xe

cargo fmt --all -- --check

cargo xbuild --release
cargo xtest --release

cargo clean --doc
cargo xdoc --no-deps --document-private-items