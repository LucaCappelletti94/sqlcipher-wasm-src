//! SQLCipher compiled into sqlite-wasm-rs reads native SQLCipher files and writes files native reads.
use sqlite_wasm_rs as ffi;
use sqlite_wasm_rs::vfs::memvfs::MemVfsUtil;
use sqlite_wasm_rs::vfs::transfer::DbTransfer;
use std::ffi::{CStr, CString};
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

struct Db(*mut ffi::sqlite3);
impl Db {
    fn open(name: &str) -> Db {
        let name = CString::new(name).unwrap();
        let mut db = std::ptr::null_mut();
        let flags = ffi::SQLITE_OPEN_READWRITE | ffi::SQLITE_OPEN_CREATE;
        assert_eq!(
            unsafe { ffi::sqlite3_open_v2(name.as_ptr(), &mut db, flags, std::ptr::null()) },
            ffi::SQLITE_OK
        );
        Db(db)
    }
    /// First column of the first row, or the error message.
    fn one(&self, sql: &str) -> Result<Option<String>, String> {
        let sql = CString::new(sql).unwrap();
        let mut stmt = std::ptr::null_mut();
        unsafe {
            if ffi::sqlite3_prepare_v2(self.0, sql.as_ptr(), -1, &mut stmt, std::ptr::null_mut())
                != ffi::SQLITE_OK
            {
                return Err(CStr::from_ptr(ffi::sqlite3_errmsg(self.0))
                    .to_string_lossy()
                    .into_owned());
            }
            let r = match ffi::sqlite3_step(stmt) {
                ffi::SQLITE_ROW => {
                    let t = ffi::sqlite3_column_text(stmt, 0);
                    Ok((!t.is_null())
                        .then(|| CStr::from_ptr(t.cast()).to_string_lossy().into_owned()))
                }
                ffi::SQLITE_DONE => Ok(None),
                _ => Err(CStr::from_ptr(ffi::sqlite3_errmsg(self.0))
                    .to_string_lossy()
                    .into_owned()),
            };
            ffi::sqlite3_finalize(stmt);
            r
        }
    }
    fn exec(&self, sql: &str) {
        let sql = CString::new(sql).unwrap();
        let rc = unsafe {
            ffi::sqlite3_exec(
                self.0,
                sql.as_ptr(),
                None,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };
        assert_eq!(rc, ffi::SQLITE_OK, "{:?}", unsafe {
            CStr::from_ptr(ffi::sqlite3_errmsg(self.0))
        });
    }
}
impl Drop for Db {
    fn drop(&mut self) {
        unsafe { ffi::sqlite3_close(self.0) };
    }
}

fn memvfs() -> MemVfsUtil {
    assert_eq!(unsafe { ffi::sqlite3_initialize() }, ffi::SQLITE_OK);
    unsafe { MemVfsUtil::get() }.unwrap()
}

#[wasm_bindgen_test]
fn sqlcipher_is_compiled_in() {
    memvfs();
    let db = Db::open("probe.db");
    db.exec(RAW);
    let v = db.one("PRAGMA cipher_version").unwrap();
    let p = db.one("PRAGMA cipher_provider").unwrap();
    assert!(
        v.as_deref()
            .is_some_and(|v| v.starts_with(&format!("{} ", sqlcipher_wasm_src::SQLCIPHER_VERSION))),
        "{v:?}"
    );
    assert_eq!(p.as_deref(), Some("libtomcrypt"));
}

#[wasm_bindgen_test]
fn reads_native_and_writes_for_native() {
    let util = memvfs();
    for (name, key) in [("native-raw.db", RAW), ("native-pass.db", PASS)] {
        util.import_db_unchecked(name, &read_file_sync(&format!("{DIR}/{name}")))
            .unwrap();
        let db = Db::open(name);
        db.exec(key);
        assert_eq!(
            db.one("SELECT v FROM t").unwrap().as_deref(),
            Some("written natively")
        );
        let wrong = Db::open(name);
        wrong.exec("PRAGMA key = 'wrong'");
        assert!(
            wrong.one("SELECT count(*) FROM sqlite_schema").is_err(),
            "{name} opened with a wrong key"
        );
    }
    for (name, key) in [("web-raw.db", RAW), ("web-pass.db", PASS)] {
        {
            let db = Db::open(name);
            db.exec(key);
            db.exec("CREATE TABLE t(v TEXT); INSERT INTO t VALUES ('written in the browser');");
        }
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
