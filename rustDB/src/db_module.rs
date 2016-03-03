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

    fn create_table(&mut self, cl_name: &str, fields: &Set<String>)->Result<&mut Collection,&'static str>{
        if self.collections.contains_key(cl_name){
            return Err("Collection name already exists.");
        }
        let mut cl = Collection::new(&fields);
        self.collections.insert(cl_name.to_owned(),cl);
        match self.collections.get_mut(cl_name){
            Some(col) =>Ok(col),
            None => Err("Database inser error"),
        }
    }

    fn find_cl(&self, cl_name: &str) -> Result<&mut Collection,&'static str>{
        match self.collections.get_mut(cl_name) {
            Some(col) => Ok(col),
            None => Err("Collection name does not exists"),
        }
    }
}


mod database_test{
    use super::{RustDB,Set};
    use vecDBCollection::{Collection,TableEntry};

    #[test]
    fn create_table_test(){
        let mut db = RustDB::new();
        let fields = new_fileds();
        let collection_for_test = Collection::new(&fields);
        let create_result = db.create_table("student",&fields);
        assert!(create_result.is_ok());
        assert_eq!(&collection_for_test,create_result.unwrap());
        let mut result_collection = create_result.unwrap();
        result_collection.insert(&new_sort_entry(0, "Ada", 24));
    }

    #[test]
    fn find_cl_test(){
        let mut db = RustDB::new();
        let fields = new_fileds();
        assert!(db.create_table("student",&fields).is_ok());
        assert!(db.find_cl("student").is_ok());
        assert!(!db.find_cl("teacher").is_ok());
    }

    fn new_fileds()->Set<String>{
        let mut fields: Set<String> = Set::new();
        fields.insert("id".to_owned());
        fields.insert("name".to_owned());
        fields.insert("age".to_owned());
        fields
    }

    fn new_sort_entry(id: usize, name: &str, age: usize) -> TableEntry{
        let mut entry = TableEntry::new();
        entry.insert("id".to_owned(), id.to_string());
        entry.insert("name".to_owned(), name.to_owned());
        entry.insert("age".to_owned(), age.to_string());
        entry
    }
}
