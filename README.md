# sqlcipher-wasm-src

[SQLCipher](https://github.com/sqlcipher/sqlcipher) with its [libtomcrypt](https://github.com/libtom/libtomcrypt) crypto provider as C source, for building SQLCipher where no system crypto library exists, such as `wasm32-unknown-unknown` through [`sqlite-wasm-rs`](https://github.com/Spxg/sqlite-wasm-rs). An experiment, not published.

```rust
let dir = sqlcipher_wasm_src::source_dir();
assert!(dir.join(sqlcipher_wasm_src::WASM_SOURCE_FILE).is_file());
assert!(dir.join(sqlcipher_wasm_src::HEADER_FILE).is_file());
```

`upgrade.sh` regenerates `sqlcipher/` from the pinned upstream releases. SQLCipher is BSD-3-Clause, SQLite is public domain, libtomcrypt is public domain or WTFPL, and the Wasm wrapper is MIT.
