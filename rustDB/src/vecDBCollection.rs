use std::sync::{Arc,Mutex};
use std::collections::{LinkedList,HashMap, BTreeSet};
use std::thread;
use std::fmt::{Display};

pub type TableEntry = HashMap<String, String>;
pub type Set<K> = BTreeSet<K>;

#[derive(Debug)]
pub struct ItemNode {
	valid: bool,
	content: TableEntry,
}

impl ItemNode {
	pub fn new(entry: &TableEntry) -> Self {
		ItemNode {
			valid: true,
			content: entry.to_owned(),
		}
	}

	pub fn is_valid(&self) -> bool {
		self.valid
	}

	pub fn set_valid(&mut self, set_value: bool){
		self.valid = set_value;
	}

	pub fn update_field(&mut self, field_name: String, field_value: String) -> Result<(), &str>{
		if let Some(x) = self.content.get_mut(&field_name) {
			*x = field_value;
			Ok(())
		}
		else {
			Err("Such field does not exist!")
		}
	}


	pub fn matched(&self, template: &TableEntry) -> bool{
		for key in template.keys() {
			if self.content.get(key) != template.get(key){
				return false;
			}
		}
		true
	}


	pub fn modify(&mut self, template: &TableEntry) {
		for key in template.keys() {
			let val = self.content.entry(key.clone()).or_insert("".to_owned());
			*val = template.get(key).unwrap().clone();
		}
	}
}

pub type EntryList = Vec<Box<ItemNode>>;

#[derive(Debug)]
pub struct Collection{
	fields: Set<String>,
	entries: EntryList,
}

impl Collection{
	pub fn new(fields: &Set<String>) -> Self {
		Collection {
			fields: fields.to_owned(),
			entries: EntryList::new()
		}
	}

	fn is_valid(&self,  target: &TableEntry) -> bool {
		for key in target.keys() {
    		if !self.fields.contains(key){
    			return false;
    		}
		}
		return true;
	}


	pub fn insert(&mut self, desired: &TableEntry){
		if self.is_valid(desired) {
			self.entries.push(Box::new(ItemNode::new(desired)));
		}
	}


	pub fn update(&mut self, target: &TableEntry, desired: &TableEntry) -> Option<usize>{
		if !self.is_valid(target) || !self.is_valid(desired) {
			None
		} else {
			let mut count = 0;
			
			for item in self.entries.iter_mut(){
				if (*item).matched(target) {
					(*item).modify(desired);
					count += 1;
				}
			}
			Some(count)
		}
	}

	pub fn find(&self, target: &TableEntry) -> Option<Vec<TableEntry>> {
		if !self.is_valid(target) {
			None
		} else {

			let mut res: Vec<TableEntry> = Vec::new();
			
			for item in &self.entries{
				if item.matched(target) {
					res.push(item.content.clone())
				}
			}
			Some(res)
		}
	}


	pub fn delete(&mut self, target: &TableEntry) -> Option<usize>{
		if !self.is_valid(target)  {
			None
		} else {

			let mut count = 0;
			let mut index = 0;

			while index < self.entries.len() {
				if self.entries[index].matched(target) {
					self.entries.remove(index);
					count += 1;
				} else {
					index += 1;
				}
			}
			Some(count)
		}
	}
}


mod itemnode_tests {
    use super::{ItemNode, TableEntry};

    #[test]
    fn node_validate_test() {
    	let mut node = ItemNode::new(&new_table_entry(0, "Ada", 24));
    	assert!(node.is_valid());
    	node.set_valid(false);
    	assert!(!node.is_valid());
    }
        
    #[test]
    fn node_matches_test() {

    	let mut node = ItemNode::new(&new_table_entry(0, "Ada", 24));
    	assert!(node.is_valid());

    	let mut matched = new_table_entry(0, "Ada", 24);
    	assert!(node.matched(&matched));
    	matched.remove(&String::from("age"));
    	assert!(node.matched(&matched));

    	
    	let mut non_matched = new_table_entry(0, "Joey", 24);
    	assert!(!node.matched(&non_matched));
    	non_matched.insert("name".to_owned(), "Ada".to_owned());
    	non_matched.insert("sex".to_owned(), "female".to_owned());
    	assert!(!node.matched(&non_matched));

    }


    #[test]
    fn node_modify_test() {

    	let mut node = ItemNode::new(&new_table_entry(0, "Ada", 24));

    	let mut matched = new_table_entry(0, "Ada", 24);
    	assert_eq!(node.content, matched);
    	
    	let mut non_matched = new_table_entry(0, "Joey", 24);
    	node.modify(&non_matched);
    	assert_eq!(node.content, non_matched);
    }

    fn new_table_entry(id: usize, name: &str, age: usize) -> TableEntry{
    	let mut entry = TableEntry::new();
    	entry.insert("id".to_owned(), id.to_string());
    	entry.insert("name".to_owned(), name.to_string());
    	entry.insert("age".to_owned(), age.to_string());
    	entry
    }
}


mod collection_tests {
    use super::{Collection, ItemNode, TableEntry, Set};

    #[test]
    fn insert_test() {

    	let mut clct = new_collection();
    	clct.insert(&new_sort_entry(0, "Ada", 24));
    	clct.insert(&new_sort_entry(1, "Joey", 25));
    	assert_eq!(clct.entries.len(), 2);

    	clct.insert(&new_sort_entry(2, "Ross", 25));
    	assert_eq!(clct.entries.len(), 3);
    }
        
    #[test]
    fn find_test() {

    	let mut clct = new_collection();
    	clct.insert(&new_sort_entry(0, "Ada", 24));
    	clct.insert(&new_sort_entry(1, "Joey", 25));
    	clct.insert(&new_sort_entry(1, "Ross", 25));

    	let mut target = TableEntry::new();
    	target.insert("age".to_owned(), 25.to_string());
    	let expected: Vec<TableEntry> = vec![new_sort_entry(1, "Joey", 25), new_sort_entry(1, "Ross", 25)];

    	assert_eq!(clct.find(&target), Some(expected));

    	let mut non_valid = new_long_entry(0, "Ada", 24, "female");
    	assert_eq!(clct.find(&non_valid), None);
    }

    #[test]
    fn update_test(){
    	let mut clct = new_collection();
    	clct.insert(&new_sort_entry(0, "Ada", 24));
    	clct.insert(&new_sort_entry(1, "Joey", 25));
    	clct.insert(&new_sort_entry(1, "Ross", 25));

    	let mut target = TableEntry::new();
    	target.insert("age".to_owned(), 25.to_string());
    	let expected: Vec<TableEntry> = vec![new_sort_entry(1, "Joey", 25), new_sort_entry(1, "Ross", 25)];
    	assert_eq!(clct.find(&target), Some(expected));

    	let mut update_desired = TableEntry::new();
    	update_desired.insert("age".to_owned(),24.to_string());
    	assert_eq!(clct.update(&target,&update_desired), Some(2));

    	let empty_vector = Vec::new();
    	assert_eq!(clct.find(&target), Some(empty_vector));

    	let mut new_target = TableEntry::new();
    	new_target.insert("age".to_owned(),24.to_string());
    	let new_expected: Vec<TableEntry> = vec![new_sort_entry(0, "Ada", 24), new_sort_entry(1, "Joey", 24), new_sort_entry(1, "Ross", 24)];
    	assert_eq!(clct.find(&new_target), Some(new_expected));
    }

    #[test]
    fn delete_test(){
    	let mut clct = new_collection();
    	clct.insert(&new_sort_entry(0, "Ada", 24));
    	clct.insert(&new_sort_entry(1, "Joey", 25));
    	clct.insert(&new_sort_entry(1, "Ross", 25));

    	let mut target = TableEntry::new();
    	target.insert("age".to_owned(), 25.to_string());
    	let expected: Vec<TableEntry> = vec![new_sort_entry(1, "Joey", 25), new_sort_entry(1, "Ross", 25)];
    	assert_eq!(clct.find(&target), Some(expected));
    	assert_eq!(clct.delete(&target), Some(2));

    	let empty_vector = Vec::new();
    	assert_eq!(clct.find(&target), Some(empty_vector));
    }

    fn new_sort_entry(id: usize, name: &str, age: usize) -> TableEntry{
    	let mut entry = TableEntry::new();
    	entry.insert("id".to_owned(), id.to_string());
    	entry.insert("name".to_owned(), name.to_owned());
    	entry.insert("age".to_owned(), age.to_string());
		entry
    }


    fn new_long_entry(id: usize, name: &str, age: usize, sex: &str) -> TableEntry{
    	let mut entry = TableEntry::new();
    	entry.insert("id".to_owned(), id.to_string());
    	entry.insert("name".to_owned(), name.to_owned());
    	entry.insert("age".to_owned(), age.to_string());
    	entry.insert("gender".to_owned(), sex.to_owned());
    	entry
    }

    fn new_collection() -> Collection {
    	let mut set: Set<String> = Set::new();
    	set.insert("id".to_owned());
    	set.insert("name".to_owned());
    	set.insert("age".to_owned());
    	Collection::new(&set)
    }
}