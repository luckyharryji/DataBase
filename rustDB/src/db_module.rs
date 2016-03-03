#[doc="
  DB interface
"]
use std::result;
use std::collections::{HashMap, BTreeSet};
use vecDBCollection::Collection;

type Set<K> = BTreeSet<K>;
type CollectionObj= HashMap<String,Collection>;

pub struct RustDB {
    collections: CollectionObj,
}

impl RustDB {
    pub fn new() -> Self{
        RustDB{
            collections: CollectionObj::new(),
        }
    }

    fn create_table(&mut self, cl_name: &str, fields: &Set<String>)->Result<&Collection,&'static str>{
        if self.collections.contains_key(cl_name){
            return Err("Collection name already exists.");
        }
        let cl = Collection::new(&fields);
        self.collections.insert(cl_name.to_owned(),cl);
        match self.collections.get(cl_name){
            Some(col) =>Ok(col),
            None => Err("Database inser error"),
        }
    }

    fn find_cl(&self, cl_name: &str) -> Result<&Collection,&'static str>{
        match self.collections.get(cl_name) {
            Some(col) => Ok(col),
            None => Err("Collection name does not exists"),
        }
    }  
}

