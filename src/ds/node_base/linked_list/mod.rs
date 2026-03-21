pub mod node;

use std::{fmt::Debug, mem::replace};

use node::Node;

#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Node<T>,
    pub length: usize,
}

impl<T: Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: Node::None,
            length: 0,
        }
    }

    pub fn read(&self, index: usize) -> &Node<T> {
        // * We begin at the first node of the list:
        let mut current_node = &self.head;
        let mut current_index = 0;

        while current_index < index {
            if let Node::Cons(_, next_node) = current_node {
                /*
                 * We keep following the links of each node until we get to the
                 * index we're looking for:
                 */
                current_node = next_node;
                current_index += 1;
            } else {
                /*
                 * If we're past the end of the list, that means the
                 * value cannot be found in the list, so return nil:
                 */
                return &Node::None;
            }
        }

        return current_node;
    }

    pub fn index_of(&self, search_value: T) -> Option<usize>
    where
        T: PartialEq,
    {
        // * We begin at the first node of the list:
        let mut current_node = &self.head;
        let mut current_index = 0;

        while let Node::Cons(value, next_node) = current_node {
            // * If we find the data we're looking for, we return it:
            if *value == search_value {
                return Some(current_index);
            }

            // * Otherwise, we move on the next node:
            current_node = next_node;
            current_index += 1;
        }

        /*
         * If we get through the entire list without finding the
         * data, we return None:
         */
        None
    }

    pub fn insert_at_index(&mut self, index: usize, value: T) {
        /*
         * If we are inserting at the beginning of the list,
         * We create the new node with the provided value
         */
        if index == 0 {
            let old_head = replace(&mut self.head, Node::None);
            /*
             * Have our new node link to what was the first node
             * and establish that our new node is now the list's first node:
             */
            self.head = Node::Cons(value, Box::new(old_head));
            self.length += 1;
            return;
        }

        // * If we are inserting anywhere other than the beginning:
        let mut current_node = &mut self.head;
        let mut current_index = 0;

        /*
         * First, we access the node immediatly before where the
         * new node will go:
         */
        while current_index < (index - 1) {
            if let Node::Cons(_, next_node) = current_node {
                current_node = next_node;
                current_index += 1;
            } else {
                return;
            }
        }

        /*
         * Have the new node link to the next node,
         * and modify the link of the previous node to point to our new node:
         */
        if let Node::Cons(_, next_node) = current_node {
            let old_next = replace(next_node.as_mut(), Node::None);
            *next_node = Box::new(Node::Cons(value, Box::new(old_next)));
            self.length += 1;
        }
    }

    pub fn delete_at_index(&mut self, index: usize) {
        // * If we are deleting the first node:
        if index == 0 {
            // * Simply set the first node to be what is currently the second node:
            let old_head = replace(&mut self.head, Node::None);
            if let Node::Cons(_, next_node) = old_head {
                self.head = *next_node;
            }
            self.length -= 1;
            return;
        }

        let mut current_node = &mut self.head;
        let mut current_index = 0;

        /*
         * First, we find the node immediatly before the one we
         * want to delete and call it current_node
         */
        while current_index < (index - 1) {
            if let Node::Cons(_, next_node) = current_node {
                current_node = next_node;
                current_index += 1;
            } else {
                return;
            }
        }

        /*
         * We change the link of the current_node to point to the
         * node_after_deleted_node, leaving the node we want
         * to delete out of the list
         */
        if let Node::Cons(_, next_node) = current_node {
            let old_next = std::mem::replace(next_node.as_mut(), Node::None);

            // * We find the node that comes after the one we're deleting:
            if let Node::Cons(_, node_after_deleted) = old_next {
                *next_node = node_after_deleted;
            }
            self.length -= 1;
        }
    }

    /*
    * 1.- Add a method to the classic LinkedList class that prints all the elements of the list.
    *
    ? My solution:
    */
    pub fn print_elements(&self) {
        let mut current_node = &self.head;
        let mut current_index = 0;
        while let Node::Cons(value, next_node) = current_node {
            if current_index != 0 {
                print!(" -> ")
            }
            print!("[{:?}]", value);
            current_node = &*next_node;
            current_index += 1;
        }
        print!("\n")
    }

    /*
    * 3.- Add a method to the classic LinkedList class that returns the last element from the list.
    * Assume you don't know how many elements are in the list.
    *
    ? My solution:
    */
    pub fn get_last_element(&self) -> Option<T>
    where
        T: Copy,
    {
        let mut current_node = &self.head;
        let mut current_index = 0;
        while let Node::Cons(value, next_node) = current_node {
            if current_index == self.length - 1 {
                return Some(value.clone());
            }

            current_node = &*next_node;
            current_index += 1;
        }

        None
    }

    /*
     * 4.- Here's a tricky one. Add a method to the classic LinkedList class that reverse the list.
     * That is, if the original list is A -> B -> C, all of the list's links should change so that C -> B -> A.
     */

    pub fn reverse_list(&mut self) {
        let mut current_node = replace(&mut self.head, Node::None);

        while let Node::Cons(value, next_node) = current_node {
            let old_head = replace(&mut self.head, Node::None);
            self.head = Node::Cons(value, Box::new(old_head));
            current_node = *next_node;
        }
    }

    /*
     * Here's a brilliant little linked list puzzle for you. Let's say you have access to a node from
     * somewhere in the middle of a classic linked list, but not the linked list itself. That is,
     * you have a variable that points to an instance of Node, but you don’t have access to the LinkedList instance.
     * In this situation, if you follow this node’s link, you can find all the items from this middle
     * node until the end, but you have no way to find the nodes that precede this node in the list.
     * Write code that will effectively delete this node from the list. The entire remaining list should
     * remain complete, with only this node removed
     */
    pub fn delete_node(node: &mut Node<T>)
    where
        T: Clone,
    {
        if let Node::Cons(value, next_node) = node {
            if let Node::Cons(next_value, next_node) = next_node.as_mut() {
                // * Overwrite current node's value with the next node's value:
                *value = next_value.clone();

                // * Skip the next node by pointing to the one after it:
                *next_node = replace(next_node, Box::new(Node::None));
            }
        }
    }
}
