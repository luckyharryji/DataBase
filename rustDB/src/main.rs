fn main() {
    println!("Hello, world!");
}


use std::fs;
use std::collections::HashMap;
use std::io;
use std::io::Error;
use std::path::{Path,PathBuf};
use std::env;
use std::sync::{Arc,Mutex};

// key-value structure goes here
type Item = HashMap<Vec<u8>, Vec<u8>>;

// could add more attribute features, so pack it into a strcut
struct DatabaseCollection {
    data_item: Item,
}

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
            	DatabaseCollection{ 
            		data_item: Item::new(),
            	}
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
}
