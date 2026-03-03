pub mod linter;
pub mod print_manager;
pub mod find_directories;
pub mod count_down_spacecraft;

use linter::{Linter, LinterTrait};

use crate::examples::{count_down_spacecraft::count_down_spacecraft, find_directories::find_direrctories, print_manager::{PrintManager, PrintManagerTrait}};

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