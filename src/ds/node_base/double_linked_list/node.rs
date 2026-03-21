use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub next_node: Option<Rc<RefCell<Node<T>>>>,
    pub previous_node: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            next_node: None,
            previous_node: None
        }
    }
}