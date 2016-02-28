#[doc="
  DB interface
"]
use std::result;
use std::collections::HashMap;
use super::collection::{Collection};

pub struct DB {
    collections: HashMap<String,Collection>,
}

impl DB {
    pub fn new() -> DB{
        DB{
            collections: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, cl_name: String, fields: Set<String>) -> Result<Collection, &'static str>{
        if self.collections.contains_key(&cl_name){
            return Err("Collection already exists.");
        }
        let cl = Collection::new(&fields);
        self.collection.insert(cl_name,cl);
        return Ok(self.collections.get(&cl_name).unwrap());
    }

    pub fn find_cl(cl_name: String) -> Option<Collection>{
        match self.collections.get(&cl_name) {
            Some(cl) => Some(cl),
            None => None,
        }
    }
}
