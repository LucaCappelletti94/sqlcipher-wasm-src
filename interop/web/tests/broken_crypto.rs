//! With crypto.getRandomValues throwing, the codec must stop rather than seed from weak randomness.
use sqlite_wasm_rs as ffi;
use sqlite_wasm_rs::vfs::memvfs::MemVfsUtil;
use std::ffi::CString;
use wasm_bindgen_test::wasm_bindgen_test;

fn exec(db: *mut ffi::sqlite3, sql: &str) -> i32 {
    let sql = CString::new(sql).unwrap();
    unsafe {
        ffi::sqlite3_exec(
            db,
            sql.as_ptr(),
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    }
}

fn open(name: &str) -> *mut ffi::sqlite3 {
    let name = CString::new(name).unwrap();
    let mut db = std::ptr::null_mut();
    let flags = ffi::SQLITE_OPEN_READWRITE | ffi::SQLITE_OPEN_CREATE;
    assert_eq!(
        unsafe { ffi::sqlite3_open_v2(name.as_ptr(), &raw mut db, flags, std::ptr::null()) },
        ffi::SQLITE_OK
    );
    db
}

#[wasm_bindgen_test]
#[should_panic(expected = "entered unreachable code")]
fn broken_web_crypto_stops_the_codec() {
    assert_eq!(unsafe { ffi::sqlite3_initialize() }, ffi::SQLITE_OK);
    unsafe { MemVfsUtil::get() }.unwrap();
    js_sys::eval("globalThis.crypto.getRandomValues = () => { throw new Error('no crypto'); }")
        .unwrap();
    // Keying activates the provider, which must abort instead of seeding Fortuna weakly.
    let db = open("keyed.db");
    exec(db, "PRAGMA key = 'secret'");
    exec(db, "CREATE TABLE t(v TEXT)");
}
