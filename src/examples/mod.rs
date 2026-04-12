pub mod linter;
pub mod print_manager;
pub mod find_directories;
pub mod count_down_spacecraft;
pub mod chapter_11;
pub mod chapter_12;
pub mod chapter_13;
pub mod chapter_14;
pub mod chapter_20;

use linter::{Linter, LinterTrait};

use crate::{ds::node_base::double_linked_list::DoubleLinkedList, examples::{chapter_11::{anagrams_of, array_sum, count_x, double_value_in_array, factorial, factorial_with_recursion, number_of_paths, reverse}, chapter_12::{fib_rec, max_2n, max_n}, chapter_13::has_duplicate_value, chapter_14::Queue, chapter_20::{greedy_algorithms::{anagram_checker::{are_anagrams, are_anagrams_2}, group_sorting::group_array, increasing_triplet::increasing_triplet, max_sum::max_sum}, sum_swap::sum_swap, two_sum::{two_sum_v_n, two_sum_v_n_2, two_sum_v_n_with_array}}, count_down_spacecraft::count_down_spacecraft, find_directories::find_direrctories, print_manager::{PrintManager, PrintManagerTrait}}};

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

pub fn test_examples_ch12() {
    println!("\n----------------- Prueba de ejemplo de obtener el numero maximo O(2^N) - ch12 --------------");
    let numbers = vec![7, 90, 9, 10, 30, 15, 20, 45];
    match max_2n(&numbers) {
        Some(number) => println!("De los numeros: {:?} el mayor es: {}", numbers.clone(), number),
        None => println!("El array no contiene elemententos o no tiene un numero mayor.")
    }

    println!("\n----------------- Prueba de ejemplo de obtener el numero maximo O(N) - ch12 --------------");
    let numbers = vec![7, 90, 9, 10, 30, 15, 20, 45];
    match max_n(&numbers) {
        Some(number) => println!("De los numeros: {:?} el mayor es: {}", numbers.clone(), number),
        None => println!("El array no contiene elemententos o no tiene un numero mayor.")
    }

    println!("\n----------------- Prueba de ejemplo de fibonacci usando recursividad - ch12 --------------");
    let number = 10;
    for i in 0..number {
        print!("{}, ", fib_rec(i));
    }
    print!("\n");
}

pub fn test_example_ch13() {
    println!("\n----------------- Prueba de ejemplo de duplicados - ch12 --------------");
    let mut arr = vec![5, 9, 3, 2, 4, 5, 6];

    print!("El array {:?}", arr.clone());
    if has_duplicate_value(&mut arr) {
        print!(" tiene duplicados.\n");
    } else {
        print!(" no tiene duplicados.\n");
    }
}

pub fn test_example_ch14() {
    let mut queue_with_double_linked_list: Queue<String> = Queue::new(DoubleLinkedList::new());
    queue_with_double_linked_list.enqueue(String::from("Prueba"));
    queue_with_double_linked_list.enqueue(String::from("de"));
    queue_with_double_linked_list.enqueue(String::from("relleno"));

    queue_with_double_linked_list.read();
    println!("El elemento eliminado es: {:?}", queue_with_double_linked_list.dequeue());
    queue_with_double_linked_list.read();

}

pub fn test_example_ch20() {
    println!("------- Prueba ejercicio two_sum del capitulo 20 --------- ");
    
    let numbers = vec![2, 0, 4, 1, 7, 9];
    println!("\nPrueba version 0(n^2): Valores: {:?}, Resultado: {}", &numbers, two_sum_v_n_2(numbers.clone()));
    println!("\nPrueba version 0(n): Valores: {:?}, Resultado: {}", &numbers, two_sum_v_n(numbers.clone()));
    println!("\nPrueba version 0(n) propia: Valores: {:?}, Resultado: {:?}", &numbers, two_sum_v_n_with_array(numbers.clone()));
    
    println!("\n------- Prueba ejercicio sum_swap del capitulo 20 --------- ");
    let array_1 = vec![5, 3, 3, 7];
    let array_2 = vec![4, 1, 1, 6];
    println!("\nPrueba de la funcion: \nValores del array 1: {:?}\nValores del array 2: {:?}, \nResultado: {:?}", &array_1, &array_2, sum_swap(array_1.clone(), array_2.clone()));
    
    println!("\n------- Prueba ejercicio max_sum del capitulo 20 --------- ");
    let greedly_numbers = vec![1, 1, 0, -3, 5];
    println!("\n------- Ejemplo 1: --------- ");
    println!("\nPrueba de la funcion: \nValores del array 1: {:?}\nResultado: {:?}", &greedly_numbers, max_sum(greedly_numbers.clone())); // ! -> 5
    
    let greedly_numbers = vec![5, -2, 3, -8, 4];
    println!("\n------- Ejemplo 2: --------- ");
    println!("\nPrueba de la funcion: \nValores del array 1: {:?}\nResultado: {:?}", &greedly_numbers, max_sum(greedly_numbers.clone())); // ! -> 6
    
    let greedly_numbers = vec![3, -4, 4, -3, 5, -9];
    println!("\n------- Ejemplo 3: --------- ");
    println!("\nPrueba de la funcion: \nValores del array 1: {:?}\nResultado: {:?}", &greedly_numbers, max_sum(greedly_numbers.clone())); // ! -> 6

    println!("\n------- Prueba ejercicio increasing_triplet del capitulo 20 --------- ");
    let stock_prices = vec![22.0, 25.0, 21.0, 18.0, 19.6, 17.0, 16.0, 20.5];
    println!("\n------- Ejemplo 1: --------- ");
    println!("\nPrueba de la funcion: \nValores del array: {:?}\nResultado: {:?}", &stock_prices, increasing_triplet(stock_prices.clone())); // ! -> true

    let stock_prices = vec![5, 2, 8, 4, 3, 7];
    println!("\n------- Ejemplo 2: --------- ");
    println!("\nPrueba de la funcion: \nValores del array: {:?}\nResultado: {:?}", &stock_prices, increasing_triplet(stock_prices.clone())); // ! -> true

    let stock_prices = vec![8, 9, 7, 10];
    println!("\n------- Ejemplo 3: --------- ");
    println!("\nPrueba de la funcion: \nValores del array: {:?}\nResultado: {:?}", &stock_prices, increasing_triplet(stock_prices.clone())); // ! -> true

    println!("\n------- Prueba ejercicio anagram_checker del capitulo 20 --------- ");
    let word_1 = String::from("rattles");
    let word_2 = String::from("startle");
    
    println!("\n------- Prueba ejercicio anagram_checker, funcion are_anagrams --------- ");
    println!("\nPrueba de la funcion: \nPalabra 1: {}\nPalabra 1: {}\n¿Son anagramas?: {}", &word_1, &word_2, are_anagrams(word_1.clone(), word_2.clone()));
    
    let word_3 = String::from("starle");
    println!("\n------- Prueba ejercicio anagram_checker, funcion are_anagrams_2 --------- ");
    println!("\nPrueba de la funcion: \nPalabra 1: {}\nPalabra 1: {}\n¿Son anagramas?: {}", &word_1, &word_3, are_anagrams_2(word_1.clone(), word_3.clone()));


    println!("\n------- Prueba ejercicio group_sorting del capitulo 20 --------- ");

    let letters_1 = vec!['d', 'd', 'd', 'c', 'c', 'c', 'a', 'a', 'a', 'b', 'b', 'b'];
    let letters_2 = vec!['d', 'd', 'd', 'c', 'c', 'c', 'a', 'a', 'a', 'b', 'b', 'b'];
    println!("\nPrueba de la funcion: \nValores desordenados: {:?}\nValores ordenados: {:?}", &letters_1, group_array(letters_1.clone()));
    println!("\nPrueba de la funcion: \nValores desordenados: {:?}\nValores ordenados: {:?}", &letters_2, group_array(letters_2.clone()));
}