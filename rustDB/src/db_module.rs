#[doc="
  DB interface
"]
use std::collections::{HashMap, BTreeSet};
use vec_dbcollection::Collection;
type Set<K> = BTreeSet<K>;
type CollectionObj= HashMap<String,Collection>;

#[derive(RustcDecodable, RustcEncodable)]
pub struct RustDB {
    collections: CollectionObj,
}

impl RustDB {
    pub fn new() -> Self{
        RustDB{
            collections: CollectionObj::new(),
        }
    }

    pub fn create_table(&mut self, cl_name: &str, fields: &Set<String>)->Result<&Collection,&'static str>{
        if self.collections.contains_key(cl_name){
            return Err("Collection name already exists.");
        }
        let cl = Collection::new(&fields);
        self.collections.insert(cl_name.to_owned(),cl);
        match self.collections.get(cl_name){
            Some(col) => {
                return Ok(col);
            },
            None => Err("Database insert error"),
        }
    }

    pub fn find_cl(&mut self, cl_name: &str) -> Result<&mut Collection,&'static str>{
        match self.collections.get_mut(cl_name) {
            Some(col) => {
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

    // pub fn get_collections(&mut self) -> Vec<&str>{
    //     let mut cls: Vec<&str> = Vec::new();
    //     for key in self.collections.keys(){
    //         cls.push(key);
    //     }
    //     cls
    // }

    pub fn find_cl_immute(&self, cl_name: &str) -> Result<&Collection,&'static str>{
        match self.collections.get(cl_name) {
            Some(col) => {
                return Ok(col);
            },
            None => Err("Collection name does not exist."),
        }
    }

    pub fn show_db(&mut self){
        for name in self.collections.keys(){
            self.show_cl(name);
        }
    }

    fn show_cl(&self, cl_name: &str){
        match self.find_cl_immute(cl_name) {
            Ok(cl) => {
                println!("**************** Collection: {} ****************", cl_name);
                for field in cl.get_fields().iter(){
                    print!("{:?}", field);
                    print!("           ");
                }
                print!("\n");
                let item_list = cl.get_entries();
                for item in item_list{
                    for field in cl.get_fields().iter() {
                        print!("{:?}", item.get_content().get(field));
                        print!("           ");
                    }
                    print!("\n");
                }
                println!("\r\n");
            }
            Err(e) => {
                println!("{:?}", e);
                println!("\r\n");
                // return false;
            },
        }
    }
}


mod database_test{
    #[allow(unused_imports)]
    use super::{RustDB,Set};
    #[allow(unused_imports)]
    use vec_dbcollection::{Collection,TableEntry};

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

    #[allow(dead_code)]
    fn new_student_fields()->Set<String>{
        let mut fields: Set<String> = Set::new();
        fields.insert("id".to_owned());
        fields.insert("name".to_owned());
        fields.insert("age".to_owned());
        fields
    }

    #[allow(dead_code)]
    fn new_other_fields()->Set<String>{
        let mut fields: Set<String> = Set::new();
        fields.insert("id".to_owned());
        fields.insert("gender".to_owned());
        fields
    }

    #[allow(dead_code)]
    fn new_sort_entry(id: usize, name: &str, age: usize) -> TableEntry{
        let mut entry = TableEntry::new();
        entry.insert("id".to_owned(), id.to_string());
        entry.insert("name".to_owned(), name.to_owned());
        entry.insert("age".to_owned(), age.to_string());
        entry
    }
}
