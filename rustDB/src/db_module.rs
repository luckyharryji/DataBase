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

    pub fn create_table(&mut self, cl_name: &str, fields: &Set<String>)->Result<&mut Collection,&'static str>{
        if self.collections.contains_key(cl_name){
            return Err("Collection name already exists.");
        }
        let mut cl = Collection::new(&fields);
        self.collections.insert(cl_name.to_owned(),cl);
        match self.collections.get_mut(cl_name){
            Some(col) => {
                println!("{:?}", col);
                return Ok(col);
            },
            None => Err("Database insert error"),
        }
    }

    pub fn find_cl(&mut self, cl_name: &str) -> Result<&mut Collection,&'static str>{
        match self.collections.get_mut(cl_name) {
            Some(col) => {
                println!("{:?}", col);
                return Ok(col);
            },
            None => Err("Collection name does not exist."),
        }
    }

    pub fn delete_cl(&mut self, cl_name: &str) -> Result<&'static str, &'static str>{
        match self.collections.contains_key(cl_name) {
            true => {
                self.collections.remove(cl_name);
                return Ok("Collection has been deleted");
            }
            false => {
                return Err("Collection does not exist.");
            }
        }
    }

    fn get_collections(&mut self) -> Vec<&str>{
        let mut cls: Vec<&str> = Vec::new();
        for key in self.collections.keys(){
            cls.push(key);
        }
        cls
    }

}


mod database_test{
    use super::{RustDB,Set};
    use vecDBCollection::{Collection,TableEntry};

    #[test]
    fn create_table_test(){
        let mut db = RustDB::new();
        let fields = new_student_fields();
        let collection_for_test = Collection::new(&fields);
        let create_result = db.create_table("student",&fields);
        assert!(create_result.is_ok());
        assert_eq!(&collection_for_test,create_result.unwrap());
    }

    #[test]
    fn find_cl_test(){
        let mut db = RustDB::new();
        let fields = new_student_fields();
        assert!(db.create_table("student",&fields).is_ok());
        assert!(db.find_cl("student").is_ok());
        assert!(!db.find_cl("teacher").is_ok());
    }
    #[test]
    fn create_table_when_table_exists() {
        let mut db = RustDB::new();
        let student_fields = new_student_fields();
        let other_fields = new_other_fields();
        assert!(db.create_table("student",&student_fields).is_ok());
        assert!(!db.create_table("student",&other_fields).is_ok());
    }
    #[test]
    fn delete_cl_test(){
        let mut db = RustDB::new();
        let student_fields = new_student_fields();
        db.create_table("student",&student_fields);
        assert!(db.delete_cl("student").is_ok());
        assert!(!db.delete_cl("student").is_ok());
    }
    #[test]
    fn create_table_after_deletion() {
        let mut db = RustDB::new();
        let student_fields = new_student_fields();
        db.create_table("student",&student_fields);
        db.delete_cl("student");
        assert!(db.create_table("student",&student_fields).is_ok());
    }

    fn new_student_fields()->Set<String>{
        let mut fields: Set<String> = Set::new();
        fields.insert("id".to_owned());
        fields.insert("name".to_owned());
        fields.insert("age".to_owned());
        fields
    }
    fn new_other_fields()->Set<String>{
        let mut fields: Set<String> = Set::new();
        fields.insert("id".to_owned());
        fields.insert("gender".to_owned());
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