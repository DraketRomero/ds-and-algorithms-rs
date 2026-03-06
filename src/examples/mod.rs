pub mod linter;
pub mod print_manager;
pub mod find_directories;
pub mod count_down_spacecraft;
pub mod chapter_11;

use linter::{Linter, LinterTrait};

use crate::examples::{chapter_11::{anagrams_of, array_sum, count_x, double_value_in_array, factorial, factorial_with_recursion, number_of_paths, reverse}, count_down_spacecraft::count_down_spacecraft, find_directories::find_direrctories, print_manager::{PrintManager, PrintManagerTrait}};

pub fn test_linter() {
    let text = String::from("(var x = { y: [1, 2, 3]})");
    let mut linter = Linter::new();

    match linter.lint(text) {
        Ok(res) => println!("El resultado fue: {}", res),
        Err(err) => println!("Se genero un error: {}", err)
    } 
}

pub fn test_print_manager() {
    let mut print_manager = PrintManager::new();
    print_manager.queue_print_job(String::from("First Document"));
    print_manager.queue_print_job(String::from("Second Document"));
    print_manager.queue_print_job(String::from("Third Document"));
    print_manager.run();
}

pub fn test_find_directories() {
    let path = String::from("./src");
    find_direrctories(path);
}

pub fn test_exercise_launch_starcraft_ch10() {
    let count_down = 10;

    println!("{}", count_down_spacecraft(count_down));
}

pub fn test_exercises_ch11() {
    println!("----------------- Prueba de ejemplo de duplicar valores en array - ch11 --------------");
    let mut values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("El array original: {:?}\nArray con valores duplicados: {:?}", values.clone(), double_value_in_array(&mut values, None));

    println!("----------------- Prueba de ejemplo de factorial normal - ch11 --------------");
    let fact = 6;
    println!("El factorial de {} es: {}", fact, factorial(fact));

    println!("----------------- Prueba de ejemplo de factorial con recursividad - ch11 --------------");
    let fact_r = 6;
    println!("El factorial de {} es: {}", fact_r, factorial_with_recursion(fact_r, None, None));
    
    println!("----------------- Prueba de ejemplo de sumar los valores del array - ch11 --------------");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("La sumatoria de {:?} es: {:?}", numbers, array_sum(&numbers));

    println!("----------------- Prueba de ejemplo de revertir caracteres - ch11 --------------");
    let text = String::from("Prueba :3");
    println!("La palabra invertida {} es: {}", text, reverse(&text));

    println!("----------------- Prueba de ejemplo de contar x's - ch11 --------------");
    let text = String::from("xEstaxesxunaxpruebax");
    println!("La palabra {} tiene {} x's", text, count_x(&text));

    println!("----------------- Prueba de ejemplo de staircase problem - ch11 --------------");
    let paths = 6;
    println!("Para {} se toman {} pasos", paths, number_of_paths(paths));

    println!("----------------- Prueba de ejemplo de anagramas - ch11 --------------");
    let text = String::from("abc");
    println!("Los anagramas de {} son:\n {:#?}", text, anagrams_of(&text));
}