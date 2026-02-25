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

/*
 * Write a function that accepts an array of strings and returns the first duplicate value it finds.
 * For example, if the array is ['a', 'b', 'c', 'd', 'c', 'e', 'f'], the function should return 'c', since it's duplicated within the array. (You can assume that there's one pair of duplicates within the array.)
 * Make sure the function has an efficienty of O(N).
 */

pub fn get_duplicates(letters: &Vec<&str>) -> Vec<String> {
    let mut new_arrray: Vec<String> = Vec::new();
    let mut duplicates: HashMap<String, i32> = HashMap::new();

    for i in 1..letters.len() - 1 {
        if duplicates.get(&letters[i].to_string()) == Some(&1) {
            new_arrray.push(letters[i].to_string());
        }

        duplicates.insert(letters[i].to_string(), 1);
    }

    new_arrray
}