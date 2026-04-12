use std::io;

use ds_and_algorithms::algorithms::Algorithms;
use ds_and_algorithms::ds::graph::test_vertices;
use ds_and_algorithms::ds::graph::weighted_graphs::test_2_weighted_graph;
use ds_and_algorithms::ds::heap::test_heap;
use ds_and_algorithms::ds::node_base::{test_double_ll, test_ll, test_search_binary_tree};
use ds_and_algorithms::ds::queue::{Queue, QueueTrait};
use ds_and_algorithms::ds::stack::{Stack, StackTrait};
use ds_and_algorithms::ds::tries::{Trie, TrieT};
use ds_and_algorithms::examples::{
    test_example_ch13, test_example_ch14, test_example_ch20, test_examples_ch12, test_exercise_launch_starcraft_ch10, test_exercises_ch11, test_find_directories, test_linter, test_print_manager
};
use ds_and_algorithms::exercises::test_exercise_4_chapter_9;
use ds_and_algorithms::exercises::{
    factorial, test_exercise_1_ch8, test_exercise_1_ch11, test_exercise_2_ch8,
    test_exercise_2_ch11, test_exercise_3_ch8, test_exercise_3_ch11, test_exercise_4_ch8,
    test_exercise_4_ch10, test_exercise_4_ch11, test_exercise_5_ch11,
};

fn test_algorithms() {
    // println!(" ---------- Bienvendio ");
    // let mut user_input = String::new();
    // let res = match read_input(&mut user_input) {
    //     Result()
    // }

    // while()
    // * Implementacion del algoritmo linear_search
    println!("\n ------- Implementacion del algoritmo linear_search -------");
    let numbers = vec![1, 20, 33, 45, 80];
    let algorithms = Algorithms::new(numbers);

    let search_value = 45;
    if let Some(pos) = algorithms.linear_search(&search_value) {
        println!(
            "El elemento {} fue encontrado en la posición: {}",
            &search_value, pos
        );
    } else {
        println!("El elemento {} no fue encontrado", &search_value);
    }

    // * Implementacion del algoritmo binary_search
    println!("\n ------- Implementacion del algoritmo binary_search --------");
    if let Some(pos) = algorithms.binary_search(&search_value) {
        println!(
            "El elemento {} fue encontrado en la posición: {}",
            &search_value, pos
        );
    } else {
        println!("El elemento {} no fue encontrado", &search_value);
    }

    // * Implementacion del algoritmo bubble sort
    println!("\n ------- Implementacion del algoritmo bubble sort --------- ");
    let vector = vec![20, 80, 75, 5, 9, 3, 14];
    let algorithms = Algorithms::new(vector.clone());
    println!("Array original: {:?}", &vector);
    println!(
        "Array ordenado con bubble sort: {:?}",
        algorithms.bubble_sort()
    );

    // * Implementacion del algoritmo selection sort
    println!("\n ------- Implementacion del algoritmo selection sort --------- ");
    let vector = vec![20, 80, 75, 5, 9, 3, 14];
    let mut algorithms = Algorithms::new(vector.clone());
    println!("Array original: {:?}", &vector);
    println!(
        "Array ordenado con selection sort: {:?}",
        algorithms.selection_sort()
    );

    // * Implementacion del algoritmo insertion sort
    println!("\n ------- Implementacion del algoritmo insertion sort --------- ");
    let vector = vec![2, 4, 7, 1, 3];
    let algorithms = Algorithms::new(vector.clone());
    println!("Array original: {:?}", &vector);
    println!(
        "Array ordenado con insertion sort: {:?}",
        algorithms.insertion_sort()
    );
    println!("\n");

    println!("\n---------- Implementacion de Quicksort / Ch13 ------------------------");
    let array = Vec::from([0, 5, 2, 1, 6, 3]);
    let mut sorteable_array = Algorithms::new(array.clone());
    sorteable_array.quicksort(0, array.len() - 1);
    println!(
        "El array antes de ser ordenado con Quicksort: {:?}\nEl array despues de ser ordenado con Quicksort: {:?}",
        array, sorteable_array.attr_reader
    );

    println!("\n---------- Implementacion de Quickselect / Ch13 ------------------------");
    let array = Vec::from([0, 50, 20, 10, 60, 30]);
    let mut sorteable_array = Algorithms::new(array.clone());

    let nth = 1;
    println!(
        "Los valores en el array son: {:?}\nEl {} valor mas alto encontrado con Quickselect: {:?}",
        array,
        nth + 1,
        sorteable_array.quickselect(nth, 0, array.len() - 1)
    );
}

fn test_exercises() {
    // test_exercise_1_ch8();
    // test_exercise_2_ch8();
    // test_exercise_3_ch8();
    // test_exercise_4_ch8();

    // test_exercise_4_chapter_9();
    // test_exercise_launch_starcraft_ch10();

    // println!("El numero es: {}\n", factorial(10));

    // test_exercise_4_ch10();

    // println!("\n ------- Solucion al ejercicio 1 del capitulo 11 --------- ");
    // test_exercise_1_ch11();

    // println!("\n ------- Solucion al ejercicio 2 del capitulo 11 --------- ");
    // test_exercise_2_ch11();

    // println!("\n ------- Solucion al ejercicio 3 del capitulo 11 --------- ");
    // test_exercise_3_ch11();

    // println!("\n ------- Solucion al ejercicio 4 del capitulo 11 --------- ");
    // test_exercise_4_ch11();

    // println!("\n ------- Solucion al ejercicio 5 del capitulo 11 --------- ");
    // test_exercise_5_ch11();
}

fn test_ds() {
    // println!("\n ------- Implementacion de una pila --------- ");
    // let mut stack: Stack<i32> = Stack::new();
    // stack.push(4);
    // stack.push(5);
    // stack.push(7);

    // println!("{:?}", stack.pop());

    // println!("\n ------- Implementacion de una cola --------- ");
    // let mut queue: Queue<i32> = Queue::new();
    // queue.enqueue(1);
    // queue.enqueue(2);
    // queue.enqueue(3);

    // println!("{:?}", queue.dequeue());

    // println!("\n ------- Implementacion de una lista ligada --------- ");
    // test_ll();

    // println!("\n ------- Implementacion de una lista doblemente ligada --------- ");
    // test_double_ll();

    // println!("\n ------- Implementacion de un arbol de busqueda binaria --------- ");
    // test_search_binary_tree();

    // println!("\n ------- Implementacion de un heap --------- ");
    // test_heap();

    // println!("\n ------- Implementacion de un Trie Search --------- ");
    // let mut trie = Trie::new();
    
    // let names = vec!["Peluche", "Peluchon", "Peluchito", "eluche", "nenuco", "nenuquito", "nenucon", "Sandia", "can", "cat"];
    
    // for i in 0..names.len() - 1 {
    //     trie.insert(names[i]);
    // }

    // println!("Valores en el array para la practica: {:?}", &names);

    // println!("\n ------- Prueba de Autocompletado  --------- ");
    // println!("{:?}", trie.autocomplete("Pel"));

    // println!("\n ------- Prueba de Autocorrector  --------- ");
    // println!("{:?}", trie.autocomplete("elu"));


    // println!("\n ------- Prueba struct Vertices --------- ");
    // test_vertices();

    println!("\n ------- Prueba struct Vertices con peso --------- ");
    test_2_weighted_graph();
}

fn test_examples() {
    // println!("\n ------- Prueba del ejemplo del linter con una pila --------- ");
    // test_linter();

    // println!("\n ------- Prueba del ejemplo de con una cola--------- ");
    // test_print_manager();

    // println!("\n ------- Prueba del ejemplo de lectura de archivos recursividad--------- ");
    // test_find_directories();

    // println!("\n ------- Prueba del ejemplo de lanzamiento de cohete --------- ");
    // test_exercise_launch_starcraft_ch10();
    
    // test_exercises_ch11();
    // test_examples_ch12();
    // test_example_ch13();
    // test_example_ch14();
    test_example_ch20();
}

fn main() {
    // test_algorithms();
    // test_exercises();
    // test_ds();
    test_examples();
}

// fn read_input(user_input: &mut String) -> Result<usize, io::Error> {
//     let input = io::stdin();

//     return input.read_line(user_input)
// }
