use ds_and_algorithms::algorithms::Algorithms;
use ds_and_algorithms::ds::stack::{Stack, StackTrait};
use ds_and_algorithms::exercises::{
    test_exercise_1_ch8, test_exercise_2_ch8, test_exercise_3_ch8, test_exercise_4_ch8,
};
use ds_and_algorithms::examples::test_linter;

fn test_algorithms() {
    // println!(" ---------- Bienvendio ");
    // let mut user_input = String::new();
    // let res = match read_input(&mut user_input) {
    //     Result()
    // }

    // while()
    // * Implementacion del algoritmo linear_search
    println!("\n ------- Implementacion del algoritmo linear_search -------");
    let Algorithms = Algorithms::new();

    let numbers = vec![1, 20, 33, 45, 80];
    let search_value = 45;

    if let Some(pos) = Algorithms::linear_search(&numbers, &search_value) {
        println!(
            "El elemento {} fue encontrado en la posición: {}",
            &search_value, pos
        );
    } else {
        println!("El elemento {} no fue encontrado", &search_value);
    }

    // * Implementacion del algoritmo binary_search
    println!("\n ------- Implementacion del algoritmo binary_search --------");
    if let Some(pos) = Algorithms::binary_search(&numbers, &search_value) {
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
    println!("Array original: {:?}", vector);
    println!(
        "Array ordenado con bubble sort: {:?}",
        Algorithms::bubble_sort(&vector)
    );

    // * Implementacion del algoritmo selection sort
    println!("\n ------- Implementacion del algoritmo selection sort --------- ");
    let mut vector = vec![20, 80, 75, 5, 9, 3, 14];
    println!("Array original: {:?}", vector.clone());
    println!(
        "Array ordenado con selection sort: {:?}",
        Algorithms::selection_sort(&mut vector)
    );

    // * Implementacion del algoritmo insertion sort
    println!("\n ------- Implementacion del algoritmo insertion sort --------- ");
    let mut vector = vec![2, 4, 7, 1, 3];
    println!("Array original: {:?}", vector.clone());
    println!(
        "Array ordenado con insertion sort: {:?}",
        Algorithms::insertion_sort(&mut vector)
    );
    println!("\n");
}

fn test_exercises() {
    test_exercise_1_ch8();
    test_exercise_2_ch8();
    test_exercise_3_ch8();
    test_exercise_4_ch8();
}

fn test_ds() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(4);
    stack.push(5);
    stack.push(7);

    println!("{:?}", stack.pop());
}

fn test_examples() {
    test_linter();
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
