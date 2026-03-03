pub mod chapter_10;
pub mod chapter_8;
pub mod chapter_9;

use crate::exercises::chapter_9::reverse_string;
use crate::exercises::chapter_10::{Element, print_numbers_from_arrays};

pub fn test_exercise_1_ch8() {
    let arr = vec![1, 2, 3, 4, 5, 6];
    let arr2 = vec![0, 2, 5, 7];

    println!(
        "Los valores del primer array son: {:?} \nY los del segundo son: {:?}\n",
        arr, arr2
    );
    println!(
        "Los elementos que cumplen la interseccion son: {:?}",
        chapter_8::intersection(&arr, &arr2)
    )
}

pub fn test_exercise_2_ch8() {
    let arr = vec!["a", "b", "c", "d", "c", "e", "f"];

    println!("\nLos valores del array son: {:?}", arr);
    println!(
        "Los elementos duplicados son: {:?}",
        chapter_8::get_duplicates(&arr)
    )
}

pub fn test_exercise_3_ch8() {
    let text = String::from("the quick brown box jumps over a lazy dog");

    println!("\nEl texto a comprobar es: {}", text);
    println!("La letra faltante es: {}", chapter_8::missing_letter(text))
}

pub fn test_exercise_4_ch8() {
    let text = String::from("minimum");

    println!("\nEl texto a comprobar es: {}", text);
    println!(
        "La primera letra no duplicada es: {}",
        chapter_8::first_non_duplicated(text)
    )
}

pub fn test_exercise_4_chapter_9() {
    let text = String::from("abcde");

    println!(
        "El texto a invertir es: {} \nInvertido es: {}",
        &text,
        reverse_string(text.clone())
    );
}

pub fn factorial(number: i32) -> i32 {
    if number == 1 {
        return 1;
    }

    return number * factorial(number - 1);
}

pub fn test_exercise_4_ch10() {
    let array = Element::Vec(vec![
        Element::Numero(1),
        Element::Numero(2),
        Element::Numero(3),
        Element::Vec(vec![
            Element::Numero(4),
            Element::Numero(5),
            Element::Numero(6),
        ]),
        Element::Numero(7),
        Element::Vec(vec![
            Element::Numero(8),
            Element::Vec(vec![
                Element::Numero(9),
                Element::Numero(10),
                Element::Numero(11),
                Element::Vec(vec![
                    Element::Numero(12),
                    Element::Numero(13),
                    Element::Numero(14),
                ]),
            ]),
            Element::Vec(vec![
                Element::Numero(15),
                Element::Numero(16),
                Element::Numero(17),
                Element::Numero(18),
                Element::Numero(19),
                Element::Vec(vec![
                    Element::Numero(20),
                    Element::Numero(21),
                    Element::Numero(22),
                    Element::Vec(vec![
                        Element::Numero(23),
                        Element::Numero(24),
                        Element::Numero(25),
                        Element::Vec(vec![
                            Element::Numero(26),
                            Element::Numero(27),
                            Element::Numero(29),
                        ]),
                    ]),
                    Element::Numero(30),
                    Element::Numero(31),
                ]),
                Element::Numero(32),
            ]),
            Element::Numero(33),
        ]),
    ]);

    print_numbers_from_arrays(&[array]);
}