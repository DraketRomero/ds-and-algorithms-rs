pub trait HeapPub<T> {
    fn new() -> Self;
    fn root_node(&self) -> &T;
    fn last_node(&self) -> &T;
    fn parent_index(index: usize) -> usize;
    fn insert(&mut self, value: T);
    fn delete(&mut self) -> Option<T>;
}

trait HeapPriv<T> {
    fn left_child_index(index: usize) -> usize;
    fn right_child_index(index: usize) -> usize;
    fn has_greater_child(&self, index: usize) -> bool;
    fn calculate_larger_child_index(&self, index: usize) -> usize;
}

pub struct Heap<T> {
    pub data: Vec<T>,
}

impl<T: Ord + Clone> HeapPub<T> for Heap<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    // * The root_node method returns the first value of the array
    fn root_node(&self) -> &T {
        &self.data[0]
    }

    // * The last_node method returns the last value of the array
    fn last_node(&self) -> &T {
        &self.data[self.data.len() - 1]
    }

    /*
     * The parent_index method accepts an index
     * and calculates the index of its parent node.
     */
    fn parent_index(index: usize) -> usize {
        return (index - 1) / 2;
    }

    fn insert(&mut self, value: T) {
        // * Turn value into last node by inserting it at the end of the array:
        self.data.push(value);

        // * Keep track of the index of the newly inserted node:
        let mut new_node_index = self.data.len() - 1;

        /*
         * The following loop executes the "trickle up" algorithm.
         *
         * If the new node is not in the root position,
         * and it's greater than its parent node:
         */
        while new_node_index > 0
            && self.data[new_node_index] > self.data[Self::parent_index(new_node_index)]
        {
            // * Swap the new node with parent node:
            let parent = Self::parent_index(new_node_index);
            self.data.swap(new_node_index, parent);

            // * Update the index of the new node:
            new_node_index = parent;
        }
    }

    fn delete(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        /*
         * We save the value we're going to delete so we can
         * return it at the end of the function:
         */
        let value_to_delete = self.data[0].clone();
        
        /*
        * We only ever delete the root node from a heap, so we
        * pop the last node from the array and make it the root node:
        */
        let last_node = self.data.pop().unwrap();
        self.data[0] = last_node;

        // ? The swap_remove method makes the same task from lines 78 - 85
        // let value_to_delete = self.data.swap_remove(0);

        // * Track the current index of the "trickle node":
        let mut trickle_node_index = 0;

        /*
         * The following loop executes the "trickle down" algorithm:
         * We run the loop as long as the trickle node has a child
         * that is greater than it:
         */
        while self.has_greater_child(trickle_node_index) {
            // * Save larger child index in a variable:
            let larger_child_index = self.calculate_larger_child_index(trickle_node_index);

            // * Swap the trickle node with its larger child:
            self.data.swap(trickle_node_index, larger_child_index);

            // * Update trickle node's new index
            trickle_node_index = larger_child_index;
        }

        Some(value_to_delete)
    }
}

impl<T: Ord> HeapPriv<T> for Heap<T> {
    /*
     * To find the left child of any node,
     * we can use the formula, (index * 2) + 1
     */
    fn left_child_index(index: usize) -> usize {
        return (index * 2) + 1;
    }

    /*
     * To find the right child of any node,
     * we can use the formula, (index * 2) + 2
     */
    fn right_child_index(index: usize) -> usize {
        return (index * 2) + 2;
    }

    fn has_greater_child(&self, index: usize) -> bool {
        /*
         * We check whether the node at index has left and right
         * children and if either of those children are greater
         * than the node at index:
         */
        let left_child_index = Self::left_child_index(index);
        let right_child_index = Self::right_child_index(index);
        let len = self.data.len();

        (left_child_index < len && self.data[left_child_index] > self.data[index])
            || (right_child_index < len && self.data[right_child_index] > self.data[index])
    }

    fn calculate_larger_child_index(&self, index: usize) -> usize {
        let left_child_index = Self::left_child_index(index);
        let right_child_index = Self::right_child_index(index);

        // * If there is no right child:
        if right_child_index >= self.data.len() {
            // * Return the left child index:
            return left_child_index;
        }

        // * If right child value is greater than left child value:
        if self.data[right_child_index] > self.data[left_child_index] {
            return right_child_index;
        }

        // * If the left child value is greater or equal to right child:
        left_child_index
    }
}
