#[derive(Debug)]
pub struct TreeNode<T> {
    pub value: T,
    pub left_child: Option<Box<TreeNode<T>>>,
    pub right_child: Option<Box<TreeNode<T>>>
}

impl<T> TreeNode<T> {
    pub fn new(data: T) -> Self {
        Self { value: data, left_child: None, right_child: None }
    }
}