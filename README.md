# sqlcipher-wasm-src

[![CI](https://github.com/LucaCappelletti94/sqlcipher-wasm-src/actions/workflows/ci.yml/badge.svg)](https://github.com/LucaCappelletti94/sqlcipher-wasm-src/actions/workflows/ci.yml)
[![CodeQL](https://github.com/LucaCappelletti94/sqlcipher-wasm-src/actions/workflows/codeql.yml/badge.svg)](https://github.com/LucaCappelletti94/sqlcipher-wasm-src/actions/workflows/codeql.yml)
[![Coverage](https://codecov.io/gh/LucaCappelletti94/sqlcipher-wasm-src/graph/badge.svg)](https://codecov.io/gh/LucaCappelletti94/sqlcipher-wasm-src)
[![Quality gate](https://sonarcloud.io/api/project_badges/measure?project=LucaCappelletti94_sqlcipher-wasm-src&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=LucaCappelletti94_sqlcipher-wasm-src)
[![SQLCipher release](https://github.com/LucaCappelletti94/sqlcipher-wasm-src/actions/workflows/sqlcipher-release.yml/badge.svg)](https://github.com/LucaCappelletti94/sqlcipher-wasm-src/actions/workflows/sqlcipher-release.yml)
[![crates.io](https://img.shields.io/crates/v/sqlcipher-wasm-src.svg)](https://crates.io/crates/sqlcipher-wasm-src)
[![docs.rs](https://docs.rs/sqlcipher-wasm-src/badge.svg)](https://docs.rs/sqlcipher-wasm-src)
[![license](https://img.shields.io/badge/license-MIT%20AND%20BSD--3--Clause%20AND%20blessing%20AND%20WTFPL-blue.svg)](https://github.com/LucaCappelletti94/sqlcipher-wasm-src/blob/main/Cargo.toml)

[SQLCipher](https://github.com/sqlcipher/sqlcipher) with its [libtomcrypt](https://github.com/libtom/libtomcrypt) crypto provider as C source, for building SQLCipher where no system crypto library exists, such as `wasm32-unknown-unknown` through [`sqlite-wasm-rs`](https://github.com/Spxg/sqlite-wasm-rs). An experiment. `rusqlite` master picks it up in the browser with no feature enabled, while `rusqlite` 0.40.2 cannot, as it takes `sqlite-wasm-rs` 0.5.

```rust
let dir = sqlcipher_wasm_src::source_dir();
assert!(dir.join(sqlcipher_wasm_src::WASM_SOURCE_FILE).is_file());
assert!(dir.join(sqlcipher_wasm_src::HEADER_FILE).is_file());
```

`upgrade.sh` regenerates `sqlcipher/` from the pinned releases, and CI checks the committed files match. It takes SQLCipher's release tag only with a valid signature from the key in `keys/sqlcipher.asc`, published on keys.openpgp.org under Stephen Lombardo's `sjlombardo@zetetic.net`, and only while the tag points at the pinned commit. It takes libtomcrypt's release archive only with a valid signature from the key in `keys/libtomcrypt.asc`, published on keyserver.ubuntu.com under Steffen Jaeckel's `s_jaeckel@gmx.de`, and only with the pinned checksum. Both keys come from those keyservers, not from the projects' own sites. A daily workflow opens a pull request for each new SQLCipher release, and a new libtomcrypt release is taken by hand.

SQLCipher is BSD-3-Clause, SQLite is public domain, libtomcrypt is public domain or WTFPL, and the Wasm wrapper is MIT.
