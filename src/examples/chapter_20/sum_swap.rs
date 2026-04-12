use std::collections::HashMap;

pub fn sum_swap(array_1: Vec<i32>, array_2: Vec<i32>) -> Option<Vec<usize>> {
    // * Hash table to store value of first array:
    let mut hash_table: HashMap<i32, usize> = HashMap::new();
    let mut sum_1 = 0;
    let mut sum_2 = 0;

    /*
     * Get sum of first array, while storing its values
     * in a hash table, together with an index
     */
    for i in 0..array_1.len() {
        let num = array_1[i];
        sum_1 += num;
        hash_table.insert(num, i);
    }

    // * Get sum of second array:
    for i in 0..array_2.len() {
        sum_2 += array_2[i];
    }

    /*
     * Calculate how much a number in the second array needs
     * to shift by. (If the difference between the two sums is odd,
     * our function can return an empty array immediately.
     * This is because there will be no whole integer in the middle 
     * of the two sums and therefore no valid swap,
     * since we assume that the input only contains integers.)
     */
    let diff_arrays = sum_1 - sum_2;
    if diff_arrays % 2 != 0 {
        return None;
    }

    let shift_amount = diff_arrays / 2;

    // * Iterate over each number in second array
    for i in 0..array_2.len() {

        /*
         * Check hash table for the number's counterpart in the 
         * first array, which is calculated as the current number
         * plus the amount it has to shift by:
         */
        let num = array_2[i];
        let counterpart = num + shift_amount;
        if hash_table.contains_key(&counterpart) {
            return Some(Vec::from([*hash_table.get(&counterpart).unwrap(), i]))
        }
    }

    None
}