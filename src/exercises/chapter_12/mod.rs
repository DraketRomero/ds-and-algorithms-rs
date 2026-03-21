/*
 * 1.- The following function accepts an array of numbers and returns the sum, as long as a particular number doesn't bring the sum above 100. If adding a particular number will make the sum higher than 100, that number is ignored. However, this function makes unnecessary recursive calls. Fix the code to eliminate the unnecessary recursion:
 */

use std::collections::HashMap;

pub fn add_until_100(arr: &[usize]) -> usize {
    if arr.len() == 0 {
        return 0;
    }

    if arr[0] + add_until_100(&arr[1..]) > 100 {
        return add_until_100(&arr[1..]);
    } else {
        return arr[0] + add_until_100(&arr[1..]);
    }
}

// ? Solution
pub fn add_until_100_sol(arr: &[usize]) -> usize {
    if arr.len() == 0 {
        return 0;
    }

    let sum = add_until_100(&arr[1..]);
    let sum_plus_first_element = arr[0] + sum;

    if sum_plus_first_element > 100 {
        return sum;
    } 

    sum_plus_first_element
}

/*
 * 2.- The following function uses recursion to calculate the Nth number from a mathematical sequence known as the "Golomb secuence". It's terribly inefficient, though! Use memoization to optimize it. (You don't have to actually understand how to Golomb secuence works to do the exercise.)
 */

pub fn golomb(position: usize) -> usize {
    if position == 1 {
        return 1;
    }

    1 + golomb(position - golomb(golomb(position - 1)))
}

// ? Solution
pub fn golomb_memo(position: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if position == 1 {
        return 1;
    }

    if !memo.contains_key(&position) {
        let inner = golomb_memo(position - 1, memo);
        let middle = golomb_memo(inner, memo);
        let result = 1 + golomb_memo(position - middle, memo);
        memo.insert(position, result);
    }

    memo[&position]
}

/*
 * 3.- Here is a solution to the "Unique Paths" problem from an exercise in the previous chapter. Use memoization to improve its efficiency:
 */

pub fn unique_paths(rows: usize, columns: usize) -> usize {
    if rows == 1 || columns == 1 {
        return 1;
    }

    unique_paths(rows - 1, columns) + unique_paths(rows, columns - 1)
}

// ? My Solution
// pub fn unique_paths_memo(rows: usize, columns: usize, memo_rows: &mut HashMap<usize, usize>, memo_cols: &mut HashMap<usize, usize>) -> usize {
//     if !memo_rows.contains_key(&rows) {
//         let result = unique_paths(rows - 1, columns);
//         memo_rows.insert(rows, result);
//     }
    
//     if !memo_cols.contains_key(&columns) {
//         let result = unique_paths(rows, columns - 1);
//         memo_cols.insert(columns, result);
//     }

//     memo_rows[&rows] + memo_cols[&columns]
// }

// ? Book's solution
pub fn unique_paths_memo(
    rows: usize,
    columns: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if rows == 1 || columns == 1 {
        return 1;
    }

    if !memo.contains_key(&(rows, columns)) {
        let result = unique_paths_memo(rows - 1, columns, memo) + unique_paths_memo(rows, columns - 1, memo);
        memo.insert((rows, columns), result);
    }

    *memo.get(&(rows, columns)).unwrap()
}