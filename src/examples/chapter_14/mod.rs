use std::fmt::Display;

use crate::ds::node_base::double_linked_list::DoubleLinkedList;

#[derive(Debug)]
pub struct Queue<T> {
    pub data: DoubleLinkedList<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new(double_linked_list: DoubleLinkedList<T>) -> Self
    where
        T: Clone,
    {
        Self {
            data: double_linked_list,
        }
    }

    pub fn enqueue(&mut self, element: T) {
        self.data.insert_at_end(element);
    }

    pub fn dequeue(&mut self) -> Option<T>
    {
        match self.data.remove_from_front() {
            Some(old_element) => Some(old_element.borrow().data.clone()),
            None => None,
        }
    }

    pub fn read(&self)
    where
        T: Display,
    {
        if self.data.length == 0 {
            println!("No hay elementos.");
        }

        let mut current_node = self.data.first_node.clone();
        let mut current_index = 0;

        while let Some(node) = current_node {
            if current_index != self.data.length && current_index != 0 {
                print!(" <-> ");
            }

            print!("[{}]", node.borrow().data);
            current_node = node.borrow().next_node.clone();
            current_index += 1;
        }
        print!("\n");
    }
}
