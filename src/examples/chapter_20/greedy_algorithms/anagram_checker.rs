/*
? Anagram Checker:
* This version of the algorithm takes a O(N * M) time complexity.
*/

use std::collections::HashMap;

pub fn are_anagrams(first_string: String, second_string: String) -> bool {
    // * Convert second_string into an array so we can delete characters from it.
    let mut second_str_arr = second_string.chars().collect::<Vec<char>>();

    for first_char in first_string.chars() {
        /*
         * If we're still iterating through the first_string, but the
         * second_str_arr is already empty:
         */
        if second_str_arr.is_empty() {
            return false;
        }

        /*
         * If we find the same character in both the first_string
         * and second_str_arr:
         */
        if let Some(index) = second_str_arr.iter().position(|c| *c == first_char) {
            /*
             * Delete the character from the second array and go back to the outer loop:
             */
            second_str_arr.remove(index);
        }
    }

    /*
     * The two string are only anagrams if the second_str_arr
     * has no characters remaining by the time we're done
     * iterating over the first_string:
     */
    second_str_arr.is_empty()
}

/*
 * This version of the algorithm takes a O(N + M) time complexity.
 */

pub fn are_anagrams_2(first_string: String, second_string: String) -> bool {
    let mut first_word_hash_table: HashMap<char, usize> = HashMap::new();
    let mut second_word_hash_table: HashMap<char, usize> = HashMap::new();

    // * Create hash table out of first string:
    for char in first_string.chars() {
        *first_word_hash_table.entry(char).or_insert(0) += 1;
    }

    // * Create hash table out of second string:
    for char in second_string.chars() {
        *second_word_hash_table.entry(char).or_insert(0) += 1; 
    }

    // * The two strings are anagrams only if the two hash tables are identical:
    return first_word_hash_table == second_word_hash_table;
}
