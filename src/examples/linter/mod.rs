use std::collections::HashMap;

use crate::ds::stack::{Stack, StackTrait};

pub trait LinterTrait<T> {
    fn new() -> Self;
    fn lint(&mut self, text: String) -> Result<String, String>;
}

trait LiterTraitPriv<T> {
    fn initialize() -> Stack<T>;
    fn is_opening_brace(brace: &str) -> bool;
    fn is_closing_brace(brace: &str) -> bool;
    fn is_not_match(opening_brace: &str, closing_brace: &str) -> bool;
}

pub struct Linter {
    stack: Stack<String>,
}

impl LiterTraitPriv<String> for Linter {
    fn initialize() -> Stack<String> {
        Stack::new()
    }

    fn is_opening_brace(brace: &str) -> bool {
        let braces = ["(", "{", "["];

        braces.contains(&brace)
    }

    fn is_closing_brace(brace: &str) -> bool {
        let braces = [")", "}", "]"];

        braces.contains(&brace)
    }

    fn is_not_match(opening_brace: &str, closing_brace: &str) -> bool {
        let braces_match: HashMap<String, String> = HashMap::from([
            (String::from("["), String::from("]")),
            (String::from("{"), String::from("}")),
            (String::from("("), String::from(")")),
        ]);

        closing_brace != braces_match[opening_brace]
    }
}

impl LinterTrait<String> for Linter {
    fn new() -> Self {
        Self {
            stack: Self::initialize(),
        }
    }

    fn lint(&mut self, text: String) -> Result<String, String> {
        for character in text.chars() {
            // * If the caracter is an opening brace
            if Self::is_opening_brace(&character.to_string()) {
                // * We push it onto the stack
                self.stack.push(character.clone().to_string());
            }
            // * If the caracter is an closing brace
            else if Self::is_closing_brace(&character.to_string()) {
                // * Pop from the stack
                let popped_opening_brace = self.stack.pop();

                /*
                 * If the stack was empty, so what we popped was nil,
                 * it means that an opening brace is missing:
                 */
                if Option::None == popped_opening_brace {
                    return Err(format!("{} doesn't have opening brace", character));
                } else {
                    /*
                     * If the popped opening brace doesn't match the
                     * current closing brace, we produce an error:
                     */
                    if Self::is_not_match(&popped_opening_brace.unwrap(), &character.to_string()) {
                        return Err(format!("{} has mismatched opening brace :c", character));
                    }
                }
            }
        }

        // * If we get to the end of line and the stack isn't empty
        if let Option::Some(character) = self.stack.read() {
            /*
             * It means we have an opening brace whithout a
             * corresponding closing brace, so we produce an error:
             */
            return Err(format!("{} does not have closing brace", character));
        }

        Ok(String::from("Everything clean :)"))
    }
}
