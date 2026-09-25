//! The generated files agree with the versions the crate declares.

use sqlcipher_src::{
    source_dir, AMALGAMATION_FILE, HEADER_FILE, LIBTOMCRYPT_VERSION, SQLCIPHER_VERSION,
    SQLITE_VERSION, WASM_BINDINGS_FILE, WASM_SOURCE_FILE,
};

fn read(file: &str) -> String {
    String::from_utf8_lossy(&std::fs::read(source_dir().join(file)).unwrap()).into_owned()
}

#[test]
fn every_generated_file_is_present() {
    for file in [
        WASM_SOURCE_FILE,
        HEADER_FILE,
        WASM_BINDINGS_FILE,
        AMALGAMATION_FILE,
        "libtomcrypt.c",
        "tomcrypt.h",
        "LICENSE-sqlcipher",
        "LICENSE-libtomcrypt",
        "SHA256SUMS",
    ] {
        assert!(source_dir().join(file).is_file(), "{file} missing");
    }
}

#[test]
fn checksums_cover_every_generated_file() {
    let mut listed: Vec<String> = read("SHA256SUMS")
        .lines()
        .map(|l| l.split_once("  ").unwrap().1.to_owned())
        .collect();
    listed.sort();
    let mut present: Vec<String> = std::fs::read_dir(source_dir())
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .filter(|name| name != "SHA256SUMS" && name != WASM_SOURCE_FILE)
        .collect();
    present.sort();
    assert_eq!(listed, present);
}

#[test]
fn sources_carry_the_declared_versions() {
    let header = read(HEADER_FILE);
    assert!(header
        .lines()
        .any(|l| l.starts_with("#define SQLITE_VERSION ")
            && l.contains(&format!("\"{SQLITE_VERSION}\""))));
    let sqlcipher = read(AMALGAMATION_FILE);
    assert!(sqlcipher
        .lines()
        .any(|l| l.trim() == format!("#define CIPHER_VERSION_NUMBER {SQLCIPHER_VERSION}")));
    let tomcrypt = read("tomcrypt.h");
    assert!(tomcrypt
        .lines()
        .any(|l| l.starts_with("#define SCRYPT")
            && l.contains(&format!("\"{LIBTOMCRYPT_VERSION}\""))));
}

#[test]
fn crate_version_encodes_the_release() {
    let mut parts = SQLCIPHER_VERSION
        .split('.')
        .map(|p| p.parse::<u64>().unwrap());
    let (major, minor, patch) = (
        parts.next().unwrap(),
        parts.next().unwrap(),
        parts.next().unwrap(),
    );
    let (numbers, metadata) = env!("CARGO_PKG_VERSION").split_once('+').unwrap();
    assert!(
        numbers.starts_with(&format!("{}.{patch}.", major * 100 + minor)),
        "{numbers}"
    );
    assert_eq!(metadata, format!("sqlcipher-{SQLCIPHER_VERSION}-sqlite-{SQLITE_VERSION}-libtomcrypt-{LIBTOMCRYPT_VERSION}"));
}

#[test]
fn sqlcipher_finalizer_is_skipped_on_wasm() {
    let sqlcipher = read(AMALGAMATION_FILE);
    let registration = sqlcipher
        .lines()
        .position(|l| l.contains("section(\".fini_array\")"))
        .expect("SQLCipher no longer registers a .fini_array finalizer, so the patch is obsolete");
    let guard = sqlcipher.lines().nth(registration - 1).unwrap();
    assert_eq!(guard.trim(), "#elif !defined(__wasm__)");
}

#[test]
fn tomcrypt_headers_are_included_by_quote() {
    let sources: Vec<String> = std::fs::read_dir(source_dir())
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .filter(|name| {
            matches!(
                std::path::Path::new(name)
                    .extension()
                    .and_then(|e| e.to_str()),
                Some("c" | "h")
            )
        })
        .collect();
    assert!(sources
        .iter()
        .any(|name| name == "tomcrypt_private.h" || name == "tomcrypt_cipher.h"));
    for file in &sources {
        assert!(
            !read(file).contains("#include <tomcrypt"),
            "{file} still includes tomcrypt by angle brackets"
        );
    }
}

#[test]
fn wasm_wrapper_keeps_its_load_bearing_settings() {
    let wrapper = read(WASM_SOURCE_FILE);
    for setting in [
        "#define SQLITE_HAS_CODEC 1",
        "#define SQLCIPHER_CRYPTO_LIBTOMCRYPT 1",
        "#define SQLITE_EXTRA_INIT sqlcipher_wasm_extra_init",
        "#define SQLITE_EXTRA_SHUTDOWN sqlcipher_extra_shutdown",
        "#define LTC_PRNG_ENABLE_LTC_RNG",
        "#define XCLOCK sqlcipher_wasm_no_clock",
        "ltc_rng = sqlcipher_wasm_rng;",
        "if (getentropy(out, len) != 0) abort();",
    ] {
        assert!(wrapper.contains(setting), "wrapper lost `{setting}`");
    }
}

#[test]
fn bindings_declare_the_codec_api_for_the_shipped_sqlite() {
    let bindings = read(WASM_BINDINGS_FILE);
    for function in ["sqlite3_key", "sqlite3_rekey"] {
        assert!(
            bindings.contains(&format!("pub fn {function}(")),
            "bindings lack `{function}`, so `SQLITE_HAS_CODEC` was not set"
        );
    }
    assert!(bindings.contains(&format!(
        "pub const SQLITE_VERSION: &::core::ffi::CStr = c\"{SQLITE_VERSION}\";"
    )));
}

#[test]
fn libc_stubs_yield_to_sqlite_wasm_rs() {
    let wrapper = read(WASM_SOURCE_FILE);
    for name in ["stdout", "stderr", "fopen", "fprintf", "rename", "atexit"] {
        let definition = wrapper
            .lines()
            .find(|l| {
                !l.starts_with("/*")
                    && (l.contains(&format!("{name}(")) || l.contains(&format!("{name} = 0;")))
            })
            .unwrap_or_else(|| panic!("no stub for `{name}`"));
        assert!(
            definition.starts_with("__attribute__((weak)) "),
            "`{name}` would clash with a sqlite-wasm-rs definition: {definition}"
        );
    }
}
