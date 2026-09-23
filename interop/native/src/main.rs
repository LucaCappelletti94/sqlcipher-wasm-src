//! write: creates native-raw.db and native-pass.db. read: opens the files the browser tests wrote.
use rusqlite::Connection;

const RAW: &str =
    "PRAGMA key = \"x'000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f'\"";
const PASS: &str = "PRAGMA key = 'correct horse battery staple'";

fn open(path: &str, key: &str) -> Connection {
    let db = Connection::open(path).unwrap();
    db.execute_batch(key).unwrap();
    db
}

fn main() {
    let dir = std::env::args().nth(2).unwrap();
    match std::env::args().nth(1).unwrap().as_str() {
        "write" => {
            for (name, key) in [("native-raw.db", RAW), ("native-pass.db", PASS)] {
                let path = format!("{dir}/{name}");
                let _ = std::fs::remove_file(&path);
                let db = open(&path, key);
                let v: String = db
                    .query_row("PRAGMA cipher_version", [], |r| r.get(0))
                    .unwrap();
                db.execute_batch(
                    "CREATE TABLE t(v TEXT); INSERT INTO t VALUES ('written natively');",
                )
                .unwrap();
                drop(db);
                let bytes = std::fs::read(&path).unwrap();
                assert!(
                    !bytes.starts_with(b"SQLite format 3"),
                    "{name} is not encrypted"
                );
                assert!(
                    !bytes.windows(16).any(|w| w == b"written natively"),
                    "{name} leaks plaintext"
                );
                println!(
                    "native SQLCipher {v} wrote {name}, {} bytes, encrypted",
                    bytes.len()
                );
            }
        }
        "read" => {
            for (name, key) in [
                ("web-raw.db", RAW),
                ("web-pass.db", PASS),
                ("rusqlite-raw.db", RAW),
                ("rusqlite-pass.db", PASS),
            ] {
                let db = open(&format!("{dir}/{name}"), key);
                let v: String = db.query_row("SELECT v FROM t", [], |r| r.get(0)).unwrap();
                assert_eq!(v, "written in the browser");
                let wrong = Connection::open(format!("{dir}/{name}")).unwrap();
                wrong.execute_batch("PRAGMA key = 'wrong'").unwrap();
                assert!(wrong
                    .query_row("SELECT count(*) FROM sqlite_schema", [], |r| r
                        .get::<_, i64>(0))
                    .is_err());
                println!("native SQLCipher read {name}: {v:?}, wrong key rejected");
            }
        }
        _ => unreachable!(),
    }
}
