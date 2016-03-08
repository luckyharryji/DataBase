use std::sync::Arc;
use std::thread;
mod storage;
use storage::RustDB;

fn main() {
    let db = RustDB::open("testdb").unwrap();
    let rc = Arc::new(db);

    let mut handles = vec![];
    for _ in 0..3 {
        let db = rc.clone();
        handles.push(thread::spawn(move || {
            assert!(db.get("test").is_none());
            db.put("test", "hello");
            assert!(db.get("test").unwrap() == b"hello");
            db.put("test", "test change");
            assert!(db.get("test").unwrap() == b"test change");
            assert!(db.delete("test") == Ok("test change".as_bytes().to_owned()));
            assert!(db.get("test").is_none());
            println!("pass");
        }));
    }

    for handle in handles {
        handle.join();
    }
}