use std::sync::{Arc,Mutex};
use std::collections::{LinkedList,HashMap, BTreeSet};
use std::sync::atomic::{AtomicPtr, Ordering};
use std::thread;


use linkedlist::{Node, List};

pub type TableEntry = HashMap<String, String>;
pub type Set<K> = BTreeSet<K>;

/*
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


	fn modify(&mut self, template: &TableEntry) {
		for key in template.keys() {
			let val = self.content.entry(key.clone()).or_insert("".to_owned());
			*val = template.get(key).unwrap().clone();
		}
	}
}
*/

pub type NodePtr = Arc<AtomicPtr<Node<TableEntry>>>; 
pub type EntryList = List<TableEntry>;


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


	fn find_tail(&self) -> Option<NodePtr> {

		if let Some(head) = self.entries.get_head() {

			let mut pre = head;
			let mut cur: Option<NodePtr>;
			loop{

				if let Some(cur) = unsafe {(*pre.load(Ordering::Relaxed)).next()} {
					pre = cur;
				} else {
					return Some(pre);
				}
			}
			
		}

		None
	}

	pub fn insert(&mut self, desired: &TableEntry){
			let node = Node::new(desired);

			loop {

				if let Some(tail) = self.find_tail(){

				} else {
					self.entries.head =
				}

			}
	}


	pub fn update(&mut self, target: &TableEntry, updated: &TableEntry) -> Option<usize>{
		if !self.is_valid(target) || self.is_valid(updated) {
			None
		} else {

			let mut count = 0;
/*			let mut it = self.entries.iter();
			while let Some(ref item) = it.next() {
				
				let peek = item.clone().into_inner().unwrap();
				if peek.matched(target){
					let shared_item = item.clone();
					let mut node = shared_item.lock().unwrap();
					if node.is_valid() {
						node.modify(updated);
						count += 1;
					}
				}
				
			}
*/
			Some(count)
		}

	}

}