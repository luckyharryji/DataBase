use std::fs::OpenOptions;
use std::sync::Arc;
use std::borrow::Cow;
use std::thread;

fn main() {
    let mut db = RustDB::open("testdb").unwrap();
    let rc = Arc::new(db);

    let mut handles = vec![];
    for i in 0..3 {
        let mut db = rc.clone();
        handles.push(thread::spawn(move || {
            assert!(db.get("test").is_none());
            db.put("test", "hello");
            assert!(db.get("test").unwrap() == b"hello");
            db.delete("test");
            assert!(db.get("test").is_none());
            println!("pass");
        }));
    }

    for handle in handles {
        handle.join();
    }
}



use std::fs;
use std::collections::HashMap;
use std::io;
use std::io::Error;
use std::path::{Path,PathBuf};
use std::env;
use std::sync::Mutex;

// key-value structure goes here
type DatabaseCollection = HashMap<Vec<u8>, Vec<u8>>;
type Records = Arc<Mutex<DatabaseCollection>>;

pub struct RustDB {
    records: Records,
}

// did not write physical disk load and storage yet
impl RustDB{
    pub fn open<P: AsRef<Path>>(path: P) -> Result<RustDB, Error> {
        Self::check_path(path).and_then(Self::create_db)
    }

    fn create_db(path: PathBuf) -> Result<RustDB, Error> {
        assert!(fs::metadata(path.as_path()).unwrap().is_dir());
        let database = RustDB {
            records: Arc::new(Mutex::new(
                DatabaseCollection::new()
            )),
        };
        Ok(database)
    }

    fn check_path<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
        let mut buf = try!(env::current_dir());
        buf = buf.join(path);
        try!(fs::create_dir_all(buf.as_path()));
        // leave retrive for later coding
        Ok(buf)
    }

    pub fn get<K: Into<Vec<u8>>>(&self, key: K)->Option<Vec<u8>>{
        let lock_data = self.records.lock().unwrap();
        lock_data.get(&key.into()).map(|value| value.clone())
    }

    pub fn put<K: Into<Vec<u8>>, V: Into<Vec<u8>>>(&self, key: K, value: V){
        let mut lock_to_write = self.records.lock().unwrap();
        lock_to_write.insert(key.into(),value.into());
    }

    pub fn delete<K: Into<Vec<u8>>>(&self, key: K) -> Result<Vec<u8>, &'static str> {
        let mut lock_to_delete = self.records.lock().unwrap();
        match lock_to_delete.remove(&key.into()) {
            Some(value) => return Ok(value),
            None => return Err("Key does not exists"),
        }
    }
}
