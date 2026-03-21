/*
 ? Exercise 1 - Ch15
 * Imagine you were to take an empty binary search tree and insert the following
 * sequence of numbers in this order: [1, 5, 9, 2, 4, 10, 6, 3, 8].
 * Draw a diagram showing what the binary search tree would look like.
 * Remember, the numbers are being inserted in the order presented here.
 * 
 ? The answer is the image Exersice_1_CH15.jpg
 */

/* 
 ? Exercise 2 - Ch15
 * The answer is 10.
 */

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
 ? Exercise 4 - Ch15
 * Imagine you were to take an empty binary search tree and insert the following
 * sequence of numbers in this order: [1, 5, 9, 2, 4, 10, 6, 3, 8].
 * Draw a diagram showing what the binary search tree would look like.
 * Remember, the numbers are being inserted in the order presented here.
 * 
 ? The answer is the image Exersice_4_CH15.jpg
 */

 /*
 ? Exercise 5 - Ch15
 * Imagine you were to take an empty binary search tree and insert the following
 * sequence of numbers in this order: [1, 5, 9, 2, 4, 10, 6, 3, 8].
 * Draw a diagram showing what the binary search tree would look like.
 * Remember, the numbers are being inserted in the order presented here.
 * 
 ? The answer is the image Exersice_5_CH15.jpg
 */

