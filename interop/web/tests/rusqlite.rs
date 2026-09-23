//! rusqlite on `wasm32-unknown-unknown` gets SQLCipher from this crate with no rusqlite feature enabled.
use rusqlite::Connection;
use sqlite_wasm_rs::vfs::memvfs::MemVfsUtil;
use sqlite_wasm_rs::vfs::transfer::DbTransfer;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(js_name = readFileSync)]
    fn read_file_sync(path: &str) -> Vec<u8>;
    #[wasm_bindgen(js_name = writeFileSync)]
    fn write_file_sync(path: &str, data: &[u8]);
}

const DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../fixtures");
const RAW: &str =
    "PRAGMA key = \"x'000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f'\"";
const PASS: &str = "PRAGMA key = 'correct horse battery staple'";

fn open(name: &str, key: &str) -> Connection {
    let db = Connection::open(name).unwrap();
    db.execute_batch(key).unwrap();
    db
}

#[wasm_bindgen_test]
fn rusqlite_reads_native_and_writes_for_native() {
    let version = Connection::open_in_memory()
        .unwrap()
        .query_row("PRAGMA cipher_version", [], |r| r.get::<_, String>(0))
        .unwrap();
    assert!(
        version.starts_with(&format!("{} ", sqlcipher_wasm_src::SQLCIPHER_VERSION)),
        "{version}"
    );
    let util = unsafe { MemVfsUtil::get() }.unwrap();
    for (name, key) in [("native-raw.db", RAW), ("native-pass.db", PASS)] {
        util.import_db_unchecked(name, &read_file_sync(&format!("{DIR}/{name}")))
            .unwrap();
        let v: String = open(name, key)
            .query_row("SELECT v FROM t", [], |r| r.get(0))
            .unwrap();
        assert_eq!(v, "written natively");
        let wrong = open(name, "PRAGMA key = 'wrong'");
        assert!(
            wrong
                .query_row("SELECT count(*) FROM sqlite_schema", [], |r| r
                    .get::<_, i64>(0))
                .is_err(),
            "{name} opened with a wrong key"
        );
    }
    for (name, key) in [("rusqlite-raw.db", RAW), ("rusqlite-pass.db", PASS)] {
        open(name, key)
            .execute_batch(
                "CREATE TABLE t(v TEXT); INSERT INTO t VALUES ('written in the browser');",
            )
            .unwrap();
        let bytes = util.export_db(name).unwrap();
        assert!(
            !bytes.starts_with(b"SQLite format 3"),
            "{name} is not encrypted"
        );
        assert!(
            !bytes.windows(22).any(|w| w == b"written in the browser"),
            "{name} leaks plaintext"
        );
        write_file_sync(&format!("{DIR}/{name}"), &bytes);
    }
}
