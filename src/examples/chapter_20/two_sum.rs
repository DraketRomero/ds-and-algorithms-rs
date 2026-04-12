use std::collections::HashMap;

/*
 ? The two sum problem
 * A continuacion se agrega el ejemplo con una complejidad de 0(N^2)
 */
pub fn two_sum_v_n_2(array: Vec<i32>) -> bool {
    for i in 0..array.len() {
        for j in  0..array.len() {
            if i != j && (array[i] + array[j]) == 10 {
                return true;
            }
        }
    }

    false
}

// * A continuacion se agrega el ejemplo con una complejidad de 0(N)
pub fn two_sum_v_n(array: Vec<i32>) -> bool {
    let mut hash_table: HashMap<i32, bool> = HashMap::new();

    for i in 0..array.len() {
        let goal = 10;
        let current = array[i];
        let counterpart = goal - current;

        /*
         * Check if the hash table contains a key which, when added
         * to the current number, would add up to 10:
         */
        if hash_table.contains_key(&counterpart) {
            return true;
        }

        // * Store each number as a key in the hash table:
        hash_table.insert(current, true);
    }

    /*
     * Return false if we get to the end of the array whitout
     * finding any number's counterpart:
     */
    false
}

// * Mi ejemplo obteniendo los valores deseados del array:
pub fn two_sum_v_n_with_array(array: Vec<i32>) -> Vec<i32> {
    let mut hash_table: HashMap<i32, bool> = HashMap::new();

    for i in 0..array.len() {
        let goal = 10;
        let current = array[i];
        let counterpart = goal - current;

        if hash_table.contains_key(&counterpart) {
            return Vec::from([current, counterpart]);
        }


        hash_table.insert(current, true);
    }

    /*
     * Return false if we get to the end of the array whitout
     * finding any number's counterpart:
     */
    Vec::new()
}