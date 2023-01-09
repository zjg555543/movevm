#!/bin/sh
set -e # Note we are not using bash here but the Alpine default shell

echo "Starting aarch64-unknown-linux-musl build"
export CC=/opt/aarch64-linux-musl-cross/bin/aarch64-linux-musl-gcc
cargo build --release --target aarch64-unknown-linux-musl --example muslc
unset CC

echo "Starting x86_64-unknown-linux-musl build"
cargo build --release --target x86_64-unknown-linux-musl --example muslc

cp target/aarch64-unknown-linux-musl/release/examples/libmuslc.a artifacts/libmovevm_muslc.aarch64.a
cp target/x86_64-unknown-linux-musl/release/examples/libmuslc.a artifacts/libmovevm_muslc.a
