use std::collections::HashMap;

/*
 * Write a function that returns the intersection of two arrays.
 * The intersection is a third array that contains all values contained within the first two arrays. For example, the intersection of [1, 2, 3, 4, 5] and [0, 2, 4, 6, 8] is [2, 4].
 *  Your function should have a complexety of O(N). (If your programming language has a built-in way of doing this, don't use it. The idea is to build the algorithm yourself.)
 */

pub fn intersection(array1: &Vec<i32>, array2: &Vec<i32>) -> Vec<i32> {
    let mut new_arrray = Vec::new();
    let mut pairs: HashMap<i32, _> = HashMap::new();

    for i in 1..array1.len() - 1 {
        pairs.insert(array1[i].clone(), true);
    }

    for i in 1..array2.len() - 1 {
        if pairs.contains_key(&array2[i]) {
            new_arrray.push(array2[i].clone());
        }
    }

    new_arrray
}