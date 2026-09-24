#![doc = include_str!("../README.md")]

use std::path::Path;

/// SQLCipher release the vendored amalgamation is generated from.
pub const SQLCIPHER_VERSION: &str = "4.19.0";

/// SQLite release that SQLCipher release is based on.
pub const SQLITE_VERSION: &str = "3.53.4";

/// libtomcrypt release providing the ciphers, hashes and random generator.
pub const LIBTOMCRYPT_VERSION: &str = "1.18.2";

/// SQLCipher amalgamation inside [`source_dir`], for native builds that pick their own crypto provider.
pub const AMALGAMATION_FILE: &str = "sqlcipher.c";

/// Public SQLCipher header inside [`source_dir`].
pub const HEADER_FILE: &str = "sqlite3.h";

/// Single translation unit inside [`source_dir`] that compiles SQLCipher with libtomcrypt for Wasm.
pub const WASM_SOURCE_FILE: &str = "sqlite3.c";

/// Rust bindings for [`HEADER_FILE`] inside [`source_dir`], from `sqlite-wasm-rs`'s bindgen setup with `SQLITE_HAS_CODEC`, so they declare `sqlite3_key` and `sqlite3_rekey`.
pub const WASM_BINDINGS_FILE: &str = "sqlcipher_bindgen.rs";

/// Directory holding the sources, at the path this crate was compiled from.
#[must_use]
pub fn source_dir() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/sqlcipher"))
}
