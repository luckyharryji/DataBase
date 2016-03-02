use std::sync::{Arc,Mutex};
use std::collections::{LinkedList,HashMap, BTreeSet};
use std::thread;
use std::fmt::{Display};

pub type TableEntry = HashMap<String, String>;
pub type Set<K> = BTreeSet<K>;

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
		return true;
	}


	pub fn modify(&mut self, template: &TableEntry) {
		for key in template.keys() {
			let val = self.content.entry(key.clone()).or_insert("".to_owned());
			*val = template.get(key).unwrap().clone();
		}
	}
}


pub type EntryList = Vec<Box<ItemNode>>;


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
			self.entries.push(ItemNode::new(desired));
		}
	}


	pub fn update(&mut self, target: &TableEntry, desired: &TableEntry) -> Option<usize>{
		if !self.is_valid(target) || !self.is_valid(desired) {
			None
		} else {
			let mut count = 0;
			for item in &self.entries{
				if item.matched(target) {
					item.modify(desired);
					count += 1;
				}
			}
			Some(count)
		}
	}

	pub fn find(&self, target: &TableEntry) -> Option<Vec<TableEntry>> {
		if !self.is_valid(target){
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
		if !self.is_valid(target){
			None
		} else {
			let mut count = 0;
			let mut index = 0;

			while index < self.entries.len() {
				let item = &self.entries[index];
				if (*item).matched(target) {
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