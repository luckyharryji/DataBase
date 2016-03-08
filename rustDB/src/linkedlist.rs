
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};

pub type Link<T> = Option<Arc<AtomicPtr<Node<T>>>>;

pub struct List<T> {
	head: Link<T>,
	tail: Link<T>,
}

impl<T> List<T>{
	pub fn new() -> Self {
		List{
			head: None,
			tail: None,
		}
	}

	pub fn get_head(&self) -> Link<T> {
		match self.head {
			Some(ref ptr) => Some(ptr.clone()),
			None => None,
		}
	}

	pub fn set_head(&self, val: T) {
		let node = Node::new(val);
		self.head = Arc::new(AtomicPtr::new(&node));

	}
}

pub struct Node<T>{
	mark: bool,
	val: T,
	next: Link<T>,
}


impl<T> Node<T> {
	pub fn new(elem: T) -> Self{
		Node{
			mark: false,
			val: elem,
			next: None,
		}
	}

	pub fn mark(&mut self) {
		self.mark = true;
	}

	pub fn next(&self) -> Link<T>{
		match self.next{
			Some(ref ptr) => Some(ptr.clone()),
			None => None,
		}
	}
}

