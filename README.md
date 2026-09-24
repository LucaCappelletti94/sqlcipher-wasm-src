# sqlcipher-wasm-src

[![CI](https://github.com/LucaCappelletti94/sqlcipher-wasm-src/actions/workflows/ci.yml/badge.svg)](https://github.com/LucaCappelletti94/sqlcipher-wasm-src/actions/workflows/ci.yml)
[![CodeQL](https://github.com/LucaCappelletti94/sqlcipher-wasm-src/actions/workflows/codeql.yml/badge.svg)](https://github.com/LucaCappelletti94/sqlcipher-wasm-src/actions/workflows/codeql.yml)
[![Coverage](https://codecov.io/gh/LucaCappelletti94/sqlcipher-wasm-src/graph/badge.svg)](https://codecov.io/gh/LucaCappelletti94/sqlcipher-wasm-src)
[![Quality gate](https://sonarcloud.io/api/project_badges/measure?project=LucaCappelletti94_sqlcipher-wasm-src&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=LucaCappelletti94_sqlcipher-wasm-src)
[![SQLCipher release](https://github.com/LucaCappelletti94/sqlcipher-wasm-src/actions/workflows/sqlcipher-release.yml/badge.svg)](https://github.com/LucaCappelletti94/sqlcipher-wasm-src/actions/workflows/sqlcipher-release.yml)
[![crates.io](https://img.shields.io/crates/v/sqlcipher-wasm-src.svg)](https://crates.io/crates/sqlcipher-wasm-src)
[![docs.rs](https://docs.rs/sqlcipher-wasm-src/badge.svg)](https://docs.rs/sqlcipher-wasm-src)
[![license](https://img.shields.io/badge/license-MIT%20AND%20BSD--3--Clause%20AND%20blessing%20AND%20WTFPL-blue.svg)](https://github.com/LucaCappelletti94/sqlcipher-wasm-src/blob/main/Cargo.toml)

[SQLCipher](https://github.com/sqlcipher/sqlcipher) with the [libtomcrypt](https://github.com/libtom/libtomcrypt) crypto provider as C source, ready for [`sqlite-wasm-rs`](https://github.com/Spxg/sqlite-wasm-rs) to compile for `wasm32-unknown-unknown`.

Point `sqlite-wasm-rs` 0.6 at the sources with `SQLITE_WASM_RS_SOURCE_DIR`, set in the environment or in `[env]` of `.cargo/config.toml`:

```sh
export SQLITE_WASM_RS_SOURCE_DIR="$(cargo metadata --format-version 1 \
  | jq -r '.packages[] | select(.name == "sqlcipher-wasm-src") | .manifest_path' \
  | xargs dirname)/sqlcipher"
cargo build --target wasm32-unknown-unknown
```

A build script gets the same directory from `source_dir()`:

```rust
let dir = sqlcipher_wasm_src::source_dir();
assert!(dir.join(sqlcipher_wasm_src::WASM_SOURCE_FILE).is_file());
assert!(dir.join(sqlcipher_wasm_src::HEADER_FILE).is_file());
```

Shared-memory builds (`+atomics`) also need `-mbulk-memory` in `CFLAGS_wasm32_unknown_unknown`.

The sources are generated from signed SQLCipher and libtomcrypt releases by `upgrade.sh`, and CI checks that files written by native SQLCipher and by this build open on both sides, in Node, Chrome and Firefox.

The wrapper is MIT, SQLCipher is BSD-3-Clause, SQLite is public domain, and libtomcrypt is public domain or WTFPL.
