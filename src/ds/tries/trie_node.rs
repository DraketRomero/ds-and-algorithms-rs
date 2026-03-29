use std::collections::HashMap;

pub trait TrieNodeTr {
    fn new() -> Self;
}

type Node = Option<Box<TrieNode>>;

#[derive(Debug, PartialEq)]
pub struct TrieNode {
    pub children: HashMap<char, Node>,
}

impl TrieNodeTr for TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
        }
    }
}