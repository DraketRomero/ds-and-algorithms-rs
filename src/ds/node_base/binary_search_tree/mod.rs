pub mod tree_node;

use std::mem::swap;

use tree_node::TreeNode;

#[derive(Debug)]
pub struct BinarySearchTree<T> {
    pub root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    // ? Exercise 3 - Ch15
    pub fn find_greatest(&self) -> Option<&T> {
        Self::find_greatest_node(&self.root)
    }

    pub fn find_greatest_node<'a>(node: &'a Option<Box<TreeNode<T>>>) -> Option<&'a T> {
        match node {
            None => None,
            Some(current) => match &current.right_child {
                None => Some(&current.value),
                Some(_) => Self::find_greatest_node(&current.right_child),
            },
        }
    }

    /*
     * 1. Designate a node to be the “current node.” (At the beginning of the algorithm, the root node is the first “current node.”)
     * 2. Inspect the value at the current node.
     * 3. If we’ve found the value we’re looking for, great!
     * 4. If the value we’re looking for is less than the current node, search for it in its left subtree.
     * 5. If the value we’re looking for is greater than the current node, search for
     * it in its right subtree.
     * 6. Repeat Steps 1 through 5 until we find the value we’re searching for, or until we hit the bottom of the tree, in which case our value must not be in the tree.
     */
    pub fn search<'a>(
        &'a self,
        tree_node: &'a Option<Box<TreeNode<T>>>,
        search_value: T,
    ) -> Option<&'a TreeNode<T>> {
        // * Base case: If the node is nonexistent...
        if let Some(child) = tree_node {
            // * And if we've found the value we're looking for, great!
            if search_value == child.value {
                return Some(child);
            }
            // * If the value we're looking for is less than the current node, search for it in this left subtree.
            else if search_value < child.value {
                return self.search(&child.left_child, search_value);
            }
            // * If the value we're looking for is less than the current node, search for it in this right subtree.
            // * search_value > child.value
            else {
                return self.search(&child.right_child, search_value);
            }
        }

        None
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            None => {
                self.root = Some(Box::new(TreeNode {
                    value,
                    left_child: None,
                    right_child: None,
                }));
            }
            Some(ref mut child) => {
                Self::insert_node(value, child);
            }
        }
    }

    pub fn insert_node(value: T, current_node: &mut Box<TreeNode<T>>) {
        if value < current_node.value {
            /*
             * If the left child does not exist, we want to insert
             * the value as the left child:
             */
            match current_node.left_child {
                None => {
                    current_node.left_child = Some(Box::new(TreeNode {
                        value,
                        left_child: None,
                        right_child: None,
                    }));
                }
                Some(ref mut child) => {
                    Self::insert_node(value, child);
                }
            }
        } else if value > current_node.value {
            /*
             * If the right child does not exist, we want to insert
             * the value as the right child:
             */

            match current_node.right_child {
                None => {
                    current_node.right_child = Some(Box::new(TreeNode {
                        value,
                        left_child: None,
                        right_child: None,
                    }));
                }
                Some(ref mut child) => {
                    Self::insert_node(value, child);
                }
            }
        }
    }

    /*
     * The complete deletion algorithm:
     *
     * 1.- If the node being deleted has no children, simply delete it.
     * 2.- If the node being has one child, delete the node and plug
     * the child into the spot where the deleted node was.
     *
     * 3.- When deleting a node with two children, replace the deleted node
     * with the successor node. The successor node is the child node whose
     * value is the least of all values that are greater than the deleted node.
     *
     * 4.- To find the successor node: visit the right child of the deleted value,
     * and then keep on visiting the left child of each subsequent child until
     * there are no more left children. The bottom value is the successor node.
     *
     * 5.- If the successor node has a right child, after plugging the successor node
     * into the spot of the deleted node, take the former right child of the successor
     * node and turn it into the left child of the former parent of the successor node.
     */

    pub fn delete(&mut self, value_to_delete: T) {
        self.root = Self::delete_node(self.root.take(), value_to_delete);
    }

    pub fn delete_node(
        tree_node: Option<Box<TreeNode<T>>>,
        value_to_delete: T,
    ) -> Option<Box<TreeNode<T>>> {
        if let Some(mut node) = tree_node {
            /*
             * If the value we're deleting is less or greather than the current node,
             * we set the left or right child respectively to be
             * the return value of a recursive call this
             * very method on the current
             * node's left or right subtree.
             */
            if value_to_delete < node.value {
                node.left_child = Self::delete_node(node.left_child, value_to_delete);

                /*
                 * We return the current node (and it's subtree if existent) to
                 * be used as the new value of its parent's left or right child:
                 */
                return Some(node);
            } else if value_to_delete > node.value {
                node.right_child = Self::delete_node(node.right_child, value_to_delete);

                return Some(node);
            }
            // * If the current node is the one we want to delete:
            else {
                /*
                 * If the current node has no left child, we delete it by
                 * returning its right child (and its subtree if existent)
                 * to be its parent's new subtree:
                 */
                if node.left_child.is_none() {
                    return node.right_child.take();
                }
                /*
                 * (If the current node has no left OR right child, this end up
                 * being None as per the first line of code in this function.)
                 */
                else if node.right_child.is_none() {
                    return node.left_child.take();
                }
                /*
                 * If the current node has two children, we delete the current node
                 * by calling the lift function (below),
                 * which changes the current node's
                 * value to the value of its successor node:
                 */
                else {
                    node.right_child = Self::lift(node.right_child, &mut node.value);

                    return Some(node);
                }
            }
        }

        /*
         * The base case is when we've hit the bottom of the tree,
         * and the parent node has no children:
         */
        None
    }

    pub fn lift(
        tree_node: Option<Box<TreeNode<T>>>,
        node_to_delete: &mut T,
    ) -> Option<Box<TreeNode<T>>> {
        if let Some(mut node) = tree_node {
            /*
             * If the current node of this function has a left child,
             * we recursively call this function to continue down
             * the left subtree to find the successor node.
             */
            if node.left_child.is_some() {
                node.left_child = Self::lift(node.left_child, node_to_delete);

                return Some(node);
            }
            /*
             * If the current node has no left child, that means the current node
             * of this function is the successor node, and we take its value
             * and make it new value of the node that we're deleting:
             */
            else {
                swap(&mut node.value, node_to_delete);

                /*
                 * We return the successor node's right to be now used
                 * as its parent's left child:
                 */
                return node.right_child.take();
            }
        }

        None
    }
}
