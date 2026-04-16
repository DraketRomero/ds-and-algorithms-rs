/*
 * You’re writing a function that accepts an array of numbers and computes
 * the highest product of any two numbers in the array. At first glance, this
 * is easy, as we can just find the two greatest numbers and multiply them.
 * However, our array can contain negative numbers and look like this:
 * [5, -10, -6, 9, 4]
 * In this case, it’s actually the product of the two lowest numbers, -10 and
 * -6 that yield the highest product of 60.
 * We could use nested loops to multiply every possible pair of numbers,
 * but this would take O(N2) time. Your job is to optimize the function so
 * that it’s a speedy O(N).
 */

fn greatest_product(array: &Vec<i32>) -> i32 {
    let mut greatest_number = i32::MIN;
    let mut second_to_greatest_number = i32::MIN;
    
    let mut lowest_number = i32::MAX;
    let mut second_to_lowest_number = i32::MAX;

    for &number in array {
        if number >= greatest_number {
            second_to_greatest_number = greatest_number;
            greatest_number = number;
        } else if number > second_to_greatest_number {
            second_to_greatest_number = number;
        }

        if number <= lowest_number {
            second_to_lowest_number = lowest_number;
            lowest_number = number;
        } else if number > lowest_number && number < second_to_lowest_number {
            second_to_lowest_number = number;
        }
    }

    let product_from_highest = greatest_number * second_to_greatest_number;
    let product_from_lowest = lowest_number * second_to_lowest_number;

    product_from_highest.max(product_from_lowest)
}

pub fn exercise_4() {
    let array = vec![-8, -6, 4, 2, 3];
    println!("\n---------- Ejercicio 3: ----------");
    println!("Los numeros son: {:?}\nEl mayor producto encontrado es: {}", &array, greatest_product(&array));
}