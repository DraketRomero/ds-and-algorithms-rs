/*
 * 1.- Given an array of positive numbers, write a function that returns the greatest product of any three numbers. The approach of using three nested loops would clock in at O(N^3), which is very slow. Use sorting to implement the function in a way that it computes at O(N log N) speed.
 * (There are even faster implementations, but we're focusing on using sorting as a technique to make code faster).
 *
 */

pub fn get_greatest_product(arr: &mut Vec<usize>) -> usize {
    arr.sort();

    return arr[arr.len() - 1] * arr[arr.len() - 2] * arr[arr.len() - 3];
}

/*
* 2.- The following function finds the "missing number" from an array of integers. That is, the array is expect to have all integers from 0 up to the array's length, but one is missing. As examples, the array, [5, 2, 4, 1, 0] is missing the number 3, and the array, [9, 3, 2, 5, 6, 7, 1, 0, 4] is missing the number 8.

* Here's an implementation that is O(N^2) (the includes method alone is already O(N), since the computer needs to search the entire array to find n):

fn find_missing_number(array: Vec<usize>) -> Option<usize> {
    for i in 0..array.len() - 1 {
        if array[i + 1] != array[i] + 1 {
            return Some(array[i] + 1);
        }
    }

    None
}

 * Use sorting to write a new implementation of this function that only takes O(N log N). (There are even faster implementations, but we're focusing on using sorting as a technique to make code faster.)
*/

pub fn find_missing_number(arr: &mut Vec<usize>) -> Option<usize> {
    arr.sort();

    for i in 0..arr.len() - 1 {
        if arr[i + 1] != arr[i] + 1 {
            return Some(arr[i] + 1);
        }
    }

    None
}
/*
 * Write three different implementations of a function that finds the greatest number within an array. Write one function that is O(N^2), one that is O(N log N) and one that is O(N).
 */

 // ? This implementation uses nested loops and is O(N^2)
pub fn max_v1(arr: Vec<usize>) -> Option<usize> {
    let mut greatest_number = None;

    for i in 0..arr.len() - 1 {
        let mut is_greatest_number = true;
        for j in 0..arr.len() - 1 {
            if arr[i] < arr[j] {
                is_greatest_number = false;
            }
        }

        if is_greatest_number {
            greatest_number = Some(i)
        }
    }
    greatest_number
}

// ? This implementation simply sorts the array and returns the last number. The sorting is O(N log N):
pub fn max_v2(arr: &mut Vec<usize>) -> Option<usize> {
    arr.sort();

    Some(arr[arr.len() - 1])
}


// ? This implementation is O(N) as we loop through the array:
pub fn max_v3(arr: Vec<usize>) -> Option<usize> {
    if arr.len() == 0 {
        return None;
    }
    
    let mut greatest_number = arr[0];

    for i in 0..arr.len() - 1 {
        if arr[i] > greatest_number {
            greatest_number = arr[i]
        }
    }

    Some(greatest_number)
}