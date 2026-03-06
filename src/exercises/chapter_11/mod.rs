/*
 * 1.- Use recursion to write a function that accepts an array of strings and returns the total number of characters across all the strings.
 * For example, if the input array is ["ab", "c", "def", "ghij"], the output should be 10 since there are 10 characters in total.
 */
pub fn count_characters(letters: Vec<String>) -> usize {
    if letters.is_empty() {
        return 0;
    }

    let first_str = &letters[0];

    if first_str.is_empty() {
        return 0;
    }

    return first_str.len() + count_characters(letters[1..].to_vec());
}

/*
 * 2.- Use recursion to write a function that accepts an array of numbers and returns a new array containing just the even numbers.
 */
pub fn even_numbers(numbers: Vec<usize>, new_vec: &mut Vec<usize>) -> Vec<usize> {
    if numbers.len() == 0 {
        return Vec::new();
    }

    let num = numbers[0];
    if num % 2 == 0 {
        new_vec.push(num);
    }  
    
    even_numbers(numbers[1..].to_vec(), new_vec);
    new_vec.to_owned()
}

/*
 * 3.- There is a numerical sequence known as "Triangular Numbers". The pattern begins as 1, 3, 6, 10, 15, 21, and continues onward with the Nth number in the pattern, which is N plus the previous number.
 * For example, the 7th number in the secuence is 28, since it's 7 (which is N) plus 21 (the previous number in the sequence).
 * Write a function that accepts a number and returns the correct number from the series. That is, if the function was passed the number 7, the function would return 28.
 */

 pub fn triangular_numbers(n: usize, prev: Option<usize>, total: Option<usize>) -> usize {
    let prev_num = prev.unwrap_or(1);
    let total_num = total.unwrap_or(0);

    if prev_num == n + 1 {
        return total_num;
    }

    return triangular_numbers(n, Some(prev_num + 1), Some(total_num + prev_num));
 }

/*
 * 4.- Use recursion to write a function that accepts a string and returns the first index that contains the character "x". For example, the string "abcdefghijklmnopqrlswxyz" has an "x" at index 23. To keep things simple assume the string definitely has at least one "x".
 */
pub fn search_x(text: String, index: Option<usize>) -> Option<usize> {
    if text.is_empty() {
        return None;
    }

    let i = index.unwrap_or(0);
    let txt = &text[i..i + 1];
    if i == text.len() {
        return None;
    }

    if txt == "x" {
        return Some(i);
    }
    
    return search_x(text, Some(i + 1));
}
/*
 * 5.- This problem is known as the "Unique Paths" problem: Let's say you have a grid of rows and columns. Write a function that accepts a number of rows and a number of columns, and calculates the number of possible "shortest" paths from upper-leftmost square to the lower-rightmost square.
 * 
 * For example, here's what the grid looks like with three rows and 7 columns. You want to get from the "S" (Start) to the "F" (Finish).
 _______________
 |S| | | | | | |
 ---------------
 | | | | | | | |
 ---------------
 | | | | | | |F|
 ---------------

 * By "shortest" path, I mean that every step, you're moving either one step to the right:
  _______________
 |S->| | | | | | |
 ---------------
 | | | | | | | |
 ---------------
 | | | | | | |F|

 * or one step downward:
  _______________
 | | | | | | | |
 ---------------
 |S| | | | | | |
 ---------------
 | | | | | | |F|

 * Again, your function should calculate the number of shortest paths.
 */

 pub fn unique_paths(rows: usize, columns: usize) -> usize {
    if rows == 1 || columns == 1 {
        return 1;
    }

    return unique_paths(rows - 1, columns) + unique_paths(rows, columns - 1);
 }