pub mod chapter_8;

pub fn test_exercise_1_ch8() {
    let arr = vec![1, 2, 3, 4, 5, 6];
    let arr2 = vec![0, 2, 5, 7];

    println!("Los valores del primer array son: {:?} \nY los del segundo son: {:?}\n", arr, arr2);
    println!("Los elementos que cumplen la interseccion son: {:?}", chapter_8::intersection(&arr, &arr2))
}

pub fn test_exercise_2_ch8() {
    let arr = vec!["a", "b", "c", "d", "c", "e", "f"];

    println!("\nLos valores del array son: {:?}", arr);
    println!("Los elementos duplicados son: {:?}", chapter_8::get_duplicates(&arr))
}

pub fn test_exercise_3_ch8() {
    let text = String::from("the quick brown box jumps over a lazy dog");

    println!("\nEl texto a comprobar es: {}", text);
    println!("La letra faltante es: {}", chapter_8::missing_letter(text))
}