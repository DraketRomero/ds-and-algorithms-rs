pub mod trie_node;

use trie_node::TrieNode;

use crate::ds::tries::trie_node::TrieNodeTr;

pub trait TrieT {
    fn new() -> Self;
    fn search(&self, word: &str) -> Option<&TrieNode>;
    fn insert(&mut self, word: &str);
    fn collect_all_words<'a>(
        &self,
        words: &'a mut Vec<String>,
        node: Option<&TrieNode>,
        word: String,
    ) -> &'a mut Vec<String>;
    fn autocomplete(&self, prefix: &str) -> Option<Vec<String>>;
    fn print_chars(&self, node: Option<&TrieNode>);
    fn autocorrect(&self, user_typo: &str) -> Vec<String>;
}

#[derive(Debug)]
pub struct Trie {
    pub root: TrieNode,
}

impl TrieT for Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    /*
    ? Trie Search
    * 1.- We establish a variable called currentNode. At the beginning of our algorithm
    * this points to the root node.

    * 2.- We iterate over each character of our search string.

    * 3.- As we point to each character of our search string, we look to see if the
    * currentNode has a child with that character as a key.

    * 4.- If it does not, we return None, as it means our search string does not exist
    * in the trie.

    * 5.- If the currentNode does have a child with the current character as the key,
    * we update the currentNode to become that child. We then go back to Step
    * 2, continuing to iterate over each character in our search string.

    * 6.- If we get to the end of our search string, it means we've found our search
    * string.
    */
    fn search(&self, word: &str) -> Option<&TrieNode> {
        let mut current_node = &self.root;

        for character in word.chars() {
            // * If the current node has a child key with current character:
            match current_node.children.get(&character) {
                // * Follow the child node:
                Some(Some(child)) => current_node = child,

                /*
                 * If the current character isn't found among
                 * the current node's children, our search word
                 * must not be in the trie:
                 */
                _ => return None,
            }
        }

        Some(current_node)
    }

    /*
    ? Trie Insert
    * 1.- We establish a variable called currentNode. At the
    * beginning of our algorithm, this points to the root node.

    * 2.- We iterate over each character of our search string.
    * Here, our search string represents the new word we're
    * inserting. We call it a search string since we're also
    * searching whether the string already exists in the trie.

    * 3.- As we point to each character of our search string,
    * we look to see if the currentNode has a child with that
    * character as a key.

    * 4.- If it does, we update the currentNode to become that
    * child node and we go back to step 2, moving on to the next
    * character of our search string.

    * 5.- If the currentNode does not have a child node that
    * matches the current character, we create such a child node
    * and update the currentNode to be this new node. We then
    * go back to step 2, moving on to the next character of our
    * search string.

    * 6.- After we insert the final character or our new word,
    * we add a '*' child to the last node to indicate the word
    * is complete.
    */
    fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for character in word.chars() {
            // * If the current node has child key with current character:
            if !current_node.children.contains_key(&character) {
                /*
                 * If the current character isn't found among
                 * the current node's children, we add
                 * the character as a new child node:
                 */

                // * Follow this new node:
                current_node
                    .children
                    .insert(character, Some(Box::new(TrieNode::new())));
            }

            // * Follow the child node:
            current_node = current_node
                .children
                .get_mut(&character)
                .unwrap()
                .as_mut()
                .unwrap();
        }

        /*
         * After inserting the entire word into the trie,
         * we add a * key at the end:
         */
        current_node.children.insert('*', None);
    }

    fn collect_all_words<'a>(
        &self,
        words: &'a mut Vec<String>,
        node: Option<&TrieNode>,
        word: String,
    ) -> &'a mut Vec<String> {
        /*
        * The first argument, words, begins as an empty array,
        * and by the end of the function will contain all the words
        * from the trie.

        * The second argument is the node whose descendants we're
        * collecting words from.

        * The third argument, word, begins as an empty string,
        * and we add character to it as we move through the trie.
        *
        * The current node is the node passed in as the first parameter,
        * or the root node if none is provided.
        */
        let current_node = node.unwrap_or(&self.root);

        // * We iterate through all the current node's children:
        for (key, child_node) in &current_node.children {
            /*
             * If the current key is *, it means we hit the end of a
             * complete word, so we can add it to our words array:
             */
            if *key == '*' {
                words.push(word.clone());
            }
            // * If we're still in the middle of a word:
            else {
                // * We recursively call this function on the child node.
                self.collect_all_words(words, child_node.as_deref(), format!("{}{}", word, key));
            }
        }

        return words;
    }

    /*
    ? Autocomplete
    * Using our search and collect_all_words methods together, we can autocomplete any prefix.
    * The searh method accepts the prefix parameter, which is the string of characteres the user
    * begins typing in.

    * First we search the trie for the existence of the prefix. If the search method doesn't find
    * the prefix in the trie, it returns None. However, if the prefix is found in the trie, the
    * method returns the node in the trie that represents the final charcater in the prefix.

    * Our autocomplete method continues by calling the collect_all_words method on the node returned
    * by the search method. This finds and collects all words that stem from that final node, which
    * represent all the complete words that can be appended to the orginial prefix to form a word.

    * Our method finally returns an array of all possible endings to the user's prefix, which we could
    * then display to the user as possible autocomplete options.
    */
    fn autocomplete(&self, prefix: &str) -> Option<Vec<String>> {
        let current_node = self.search(prefix)?;

        let mut wrds: Vec<String> = Vec::new();
        self.collect_all_words(&mut wrds, Some(current_node), prefix.to_string());

        Some(wrds)
    }


    // ? Exercises
    fn print_chars(&self, node: Option<&TrieNode>) {
        let current_node = node.unwrap_or(&self.root);

        for (key, child_node) in &current_node.children {
            println!("{}", key);

            if *key != '*' {
                self.print_chars(child_node.as_deref());
            }
        }
    }

    fn autocorrect(&self, user_typo: &str) -> Vec<String> {
        let mut current_node = &self.root;

        let mut word_found_so_far = String::new();

        for character in user_typo.chars() {
            if let Some(letter) = current_node.children.get(&character) {
                word_found_so_far.push(character);
                current_node = letter.as_deref().unwrap()
            } else {
                let mut words: Vec<String> = Vec::new();
                self.collect_all_words(&mut words, Some(current_node), word_found_so_far);
                return words;
            }
        }

        return vec![user_typo.to_string()];
    }
}
