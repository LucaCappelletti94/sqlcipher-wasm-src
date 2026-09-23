#!/bin/sh -e

# Native SQLCipher writes, the wasm build reads and writes back, native reads the result.
cd "$(dirname "$0")"
FIXTURES="$(pwd)/fixtures"
rm -rf "$FIXTURES" && mkdir -p "$FIXTURES"
cargo run --release --manifest-path native/Cargo.toml -- write "$FIXTURES"
(cd web && SQLITE_WASM_RS_SOURCE_DIR="$(pwd)/../../sqlcipher" wasm-pack test --node --release)
cargo run --release --manifest-path native/Cargo.toml -- read "$FIXTURES"
