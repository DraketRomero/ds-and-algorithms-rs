pub fn has_duplicate_value<T: Ord>(array: &mut Vec<T>) -> bool {
    /*
     * Presort the array:
     * (In Javascript, the following usage of the sort function
     * is required to ensure that the numbers are in numerical order,
     * instead of alphabetical order.) 
     */
    array.sort();

    // * Iterate through the values in the array up until the last one:
    for i in 0..array.len() - 1 {

        /*
         * If the value is identical to the next value
         * in the array, we found a duplicate:
         */
        if array[i] == array[i + 1] {
            return true;
        }
    }

    /*
     * If we get to the end of the array without having
     * returned true, it means there are no duplicates.
     */
    return false;
}