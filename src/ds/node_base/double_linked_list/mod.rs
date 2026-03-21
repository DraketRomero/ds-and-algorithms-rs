use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::ds::node_base::double_linked_list::node::Node;

pub mod node;

#[derive(Debug, Clone)]
pub struct DoubleLinkedList<T> {
    pub first_node: Option<Rc<RefCell<Node<T>>>>,
    pub last_node: Option<Rc<RefCell<Node<T>>>>,
    pub length: usize,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            first_node: None,
            last_node: None,
            length: 0,
        }
    }

    pub fn insert_at_end(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));

        match self.last_node.take() {
            // * If there are no elements yet in the linked_list
            None => {
                self.first_node = Some(Rc::clone(&new_node));
                self.last_node = Some(new_node)
            }
            // * If the linked list already has at least one node:
            Some(old_node) => {
                new_node.borrow_mut().previous_node = Some(Rc::downgrade(&old_node));
                old_node.borrow_mut().next_node = Some(Rc::clone(&new_node));
                self.last_node = Some(new_node);
            }
        }

        self.length += 1;
    }

    pub fn remove_from_front(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
        let old_node = self.first_node.take().map(|old_node| {
            match old_node.borrow_mut().next_node.take() {
                Some(new_node) => {
                    new_node.borrow_mut().previous_node = None;
                    self.first_node = Some(new_node);
                }
                None => {
                    self.last_node = None;
                }
            }

            self.length -= 1;
            old_node
        });

        old_node
    }

    pub fn print_from_firs_to_last_node(&self)
    where
        T: Display,
    {
        let mut current_node = self.first_node.clone();
        let mut current_index = 0;
        while let Some(node) = current_node {
            if current_index != self.length && current_index != 0 {
                print!(" <-> ");
            }

            print!("[{}]", node.borrow().data);
            current_node = node.borrow().next_node.clone();
            current_index += 1;
        }
        print!("\n");
    }

    /*
    * 2.- Add a method to the DoublyLinkedList class that prints all the elements of the list in reverse order.
    *
    ? My solution:
    */
    pub fn print_from_last_to_first_node(&self)
    where
        T: Display,
    {
        let mut current_node = self.last_node.clone();
        let mut current_index = self.length;

        while let Some(node) = current_node {
            if current_index != self.length {
                print!(" <-> ");
            }

            print!("[{}]", node.borrow().data);
            current_node = node
                .borrow()
                .previous_node
                .as_ref()
                .and_then(|weak| weak.upgrade());
            current_index -= 1;
        }
        print!("\n");
    }
}
