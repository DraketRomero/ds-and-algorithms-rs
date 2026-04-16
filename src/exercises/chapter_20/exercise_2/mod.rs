/*
 * You’re writing a function that accepts an array of distinct integers from
 * 0, 1, 2, 3…up to N. However, the array will be missing one integer, and
 * your function is to return the missing one.
 * For example, this array has all the integers from 0 to 6, but is missing
 * the 4:
 * [2, 3, 0, 6, 1, 5]
 * Therefore, the function should return 4.
 * The next example has all the integers from 0 to 9, but is missing the 1:
 * [8, 2, 3, 9, 4, 7, 5, 0, 6]
 * In this case, the function should return the 1.
 * Using a nested-loops approach would take up to O(N2). Your job is to
 * optimize the code so that it has a runtime of O(N).
 */

fn find_missing_number(array: &Vec<i32>) -> i32 {
    let full_sum: i32 = (1..=array.len() as i32).sum();
    let current_sum: i32 = array.iter().sum();

    full_sum - current_sum
}

pub fn exercise_2() {
    println!("\n---------- Ejercicio 2: ----------");
    println!("\nEjemplo 1:");
    let numbers = vec![2, 3, 0, 6, 1, 5];
    println!(
        "Los numeros son: {:?}\nEl valor faltante es: {}",
        &numbers,
        find_missing_number(&numbers)
    );

    println!("\nEjemplo 2:");
    let numbers = vec![8, 2, 3, 9, 4, 7, 5, 0, 6];
    println!(
        "Los numeros son: {:?}\nEl valor faltante es: {}",
        &numbers,
        find_missing_number(&numbers)
    );
}
