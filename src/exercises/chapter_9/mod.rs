use crate::ds::stack::{Stack, StackTrait};

pub fn reverse_string(text: String) -> String {
    let mut stack: Stack<String> = Stack::new();

    let mut reversed = String::new();

    for c in text.chars() {
        stack.push(c.to_string());
    }

    while let Some(letter) = stack.pop() {
        reversed.push_str(&letter);
    }
    reversed
}