/*
* Exercises solved of chapter 17
*
? 1.- List all the words stored in the following trie:

   1.- Tag
   2.- Tan
   3.- Tank
   4.- Tap
   5.- Today
   6.- Total
   7.- We
   8.- Well
   9.- Went

? 2.- Draw a trie that stores the following words: “get”, “go”, “got”,
? “gotten”, “hall”, “ham”, “hammer”, “hill”, and “zebra”.
* The solution to this problem is on image Exersice_2_CH17.jpg

? 3.- Write a function that traverses each node of a trie and prints each key,
? including all "*" keys.
*/
// fn print_chars(&self, node: Option<&TrieNode>) {
//     let current_node = node.unwrap_or(&self.root);

//     for (key, child_node) in &current_node.children {
//         println!("{}", key);

//         if *key != '*' {
//             self.print_chars(child_node.as_deref());
//         }
//     }
// }

/*
? 4.- Write an autocorrect function that attempts to replace a user’s typo with
? a correct word. The function should accept a string that represents text
? a user typed in. If the user’s string is not in the trie, the function should
? return an alternative word that shares the longest possible prefix with
? the user’s string.

?  For example, let’s say our trie contained the words “cat,” “catnap,” and
? “catnip.” If the user accidentally types in “catnar,” our function should
? return “catnap,” since that’s the word from the trie that shares the longest
? prefix with “catnar.” This is because both “catnar” and “catnap” share a
? prefix of “catna,” which is five characters long. The word “catnip” isn’t as
? good, since it only shares the shorter, four-character prefix of “catn” with
? “catnar.”
? One more example: if the user types in “caxasfdij,” the function could
? return any of the words “cat,” “catnap,” and “catnip,” since they all share
? the same prefix of “ca” with the user’s typo.
? If the user’s string is found in the trie, the function should just return
? the word itself. This should be true even if the user’s text is not a complete
? word, as we’re only trying to correct typos, not suggest endings to the
? user’s prefix.
*/
// fn autocorrect(&self, user_typo: &str) -> Vec<String> {
//     let mut current_node = &self.root;

//     let mut word_found_so_far = String::new();

//     for character in user_typo.chars() {
//         if let Some(letter) = current_node.children.get(&character) {
//             word_found_so_far.push(character);
//             current_node = letter.as_deref().unwrap()
//         } else {
//             let mut words: Vec<String> = Vec::new();
//             self.collect_all_words(&mut words, Some(current_node), word_found_so_far);
//             return words;
//         }
//     }

//     return vec![user_typo.to_string()];
// }
