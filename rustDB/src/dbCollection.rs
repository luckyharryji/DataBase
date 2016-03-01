use std::sync::{Arc,Mutex}
use std::collections::{LinkedList,HashMap, BtreeSet};
use std::thread;
use std::fmt::{Display};

pub type TableEntry = HashMap<String, String>,
pub type Set<K> = BtreeSet<K>;

pub struct ItemNode {
	valid: bool,
	content: TableEntry,
}

impl ItemNode {
	pub fn new(entry: &TableEntry) -> Self {
		ItemNode {
			valid: true,
			content: entry,
		}
	}

	pub fn is_valid(&self) -> bool {
		self.valid
	}

	pub fn set_valid(&self, set_value: bool){
		self.valid = set_value;
	}

	pub fn update_field(&self, field_name: String, field_value: String) -> Result<_, &str>{
		if let Some(x) = self.content.get_mut(&field_name) {
			*x = field_value;
			Ok(_)
		}
		else {
			Err("Such field does not exist!")
		}
	}
}


pub EntryList = LinkedList<Box<ItemNode>>;

pub struct Collection{
	fields: Set<String>,
	entries: EntryList,
}

impl Collection{
	pub fn new(fields: &Set<String>) -> Self {
		fields: fields,
		entries: EntryList::new(),
	}

}