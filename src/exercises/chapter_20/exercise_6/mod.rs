/*
 * You’re writing a function that accepts an array of unsorted integers and
 * returns the length of the longest consecutive sequence among them. The
 * sequence is formed by integers that increase by 1. For example, in the
 * array:
 * [10, 5, 12, 3, 55, 30, 4, 11, 2]
 * the longest consecutive sequence is 2-3-4-5. These four integers form an
 * increasing sequence because each integer is one greater than the previous
 * one. While there’s also a sequence of 10-11-12, it’s only a sequence of
 * three integers. In this case, the function should return 4, since that’s the
 * length of the longest consecutive sequence that can be formed from this
 * array.
 * One more example:
 * [19, 13, 15, 12, 18, 14, 17, 11]
 * This array’s longest sequence is 11-12-13-14-15, so the function would
 * return 5.
 * If we sorted the array, we can then traverse the array just once to find
 * the longest consecutive sequence. However, the sorting itself would take
 * O(N log N). Your job is to optimize the function so that it takes O(N) time.
 * report.
 */

use std::collections::HashMap;

 fn longest_sequence_length(temperatures: Vec<i32>) -> i32 {
    let mut hash_table = HashMap::new();
    let mut greatest_sequence_length = 0;

    for i in &temperatures {
        hash_table.insert(*i, true);
    }

    for number in temperatures {
        if !hash_table.contains_key(&(number - 1)) {
            let mut current_secuence_length = 1;
            let mut current_number = number;

            while hash_table.contains_key(&(&current_number + 1)) {
                current_number += 1;
                current_secuence_length += 1;

                if current_secuence_length > greatest_sequence_length {
                    greatest_sequence_length = current_secuence_length;
                }
            }
        }
    }

    greatest_sequence_length
}

pub fn exercise_6() {
    println!("\n---------- Ejercicio 6: ----------");
    println!("\n Ejemplo 1: ");
    let first_sequence = vec![10, 5, 12, 3, 55, 30, 4, 11, 2];
    println!("Los numeros son: {:?}\nLa longitud de la secuencia mas larga es: {:?}", &first_sequence, longest_sequence_length(first_sequence.clone()));
    
    println!("\n Ejemplo 2: ");
    let second_sequence = vec![19, 13, 15, 12, 18, 14, 17, 11];
    println!("Los numeros son: {:?}\nLa longitud de la secuencia mas larga es: {:?}", &second_sequence, longest_sequence_length(second_sequence.clone()));
}