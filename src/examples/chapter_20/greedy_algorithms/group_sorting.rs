use std::collections::HashMap;
use std::iter::repeat;

pub fn group_array(letters: Vec<char>) -> Vec<char> {
    let mut hash_table: HashMap<char, usize> = HashMap::new();
    let mut new_array: Vec<char> = Vec::new();

    // * Store the tallies of each string in a hash table:
    for char in &letters {
        *hash_table.entry(*char).or_insert(0) += 1; 
    }

    /*
     * Iterate over the hash table and populate a new array 
     * with the correct number of each string:
     */
    for (key, count) in &hash_table {
        // for _ in 0..*count {
        //     new_array.push(*key);
        // }

        // ? repeat crea un iterador que repite el valor count veces:
        new_array.extend(repeat(*key).take(*count));
    }

    new_array
}