pub mod linkedlist;

use std::sync::atomic::{AtomicPtr, Ordering};

type Link<T> = Option<AtomicPtr<Node<T>>>;

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
}

