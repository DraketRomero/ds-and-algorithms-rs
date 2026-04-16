/*
 * You’re creating software that analyzes the data of body temperature
 * readings taken from hundreds of human patients. These readings are
 * taken from healthy people and range from 97.0 degrees Fahrenheit to
 * 99.0 degrees Fahrenheit. An important point: within this application, the
 * decimal point never goes beyond the tenths place.
 * Here’s a sample array of temperature readings:
 * [98.6, 98.0, 97.1, 99.0, 98.9, 97.8, 98.5, 98.2, 98.0, 97.1]
 * You are to write a function that sorts these readings from lowest to highest.
 * If you used a classic sorting algorithm such as Quicksort, this would take
 * O(N log N). However, in this case, it’s actually possible to write a faster
 * sorting algorithm.
 * Yes, that’s right. Even though you’ve learned that the fastest sorts are
 * O(N log N), this case is different. Why? In this case, there’s a limited
 * number of possibilities of what the readings will be. In such a case, we
 * can sort these values in O(N). It may be N multiplied by a constant, but
 * that’s still considered O(N).
*/

use std::collections::HashMap;

fn sort_human_body_temperatures(temperatures: &Vec<f64>) -> Vec<f64> {
    let mut hash_table = HashMap::new();
    let mut sorted = Vec::new();

    for temperature in temperatures.iter() {
        let count = hash_table.entry(temperature.to_bits()).or_insert(0);
        *count += 1;
    }

    let mut temperature = 970;

    while temperature <= 990 {
        let key = (temperature as f64 / 10.0).to_bits();

        if let Some(&count) = hash_table.get(&key) {
            for _ in 0..count {
                sorted.push(temperature as f64 / 10.0);
            }
        }

        temperature += 1
    }
    
    sorted
}

pub fn exercise_5() {
    let temperatures = vec![98.6, 98.0, 97.1, 99.0, 98.9, 97.8, 98.5, 98.2, 98.0, 97.1];
    println!("\n---------- Ejercicio 5: ----------");
    println!("Las temperaturas son: {:?}\nDespues de ser ordenadas: {:?}", &temperatures, sort_human_body_temperatures(&temperatures));
}