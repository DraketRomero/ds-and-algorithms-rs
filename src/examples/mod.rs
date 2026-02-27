pub mod linter;

use linter::{Linter, LinterTrait};

pub fn test_linter() {
    let text = String::from("(var x = { y: [1, 2, 3]})");
    let mut linter = Linter::new();

    match linter.lint(text) {
        Ok(res) => println!("El resultado fue: {}", res),
        Err(err) => println!("Se genero un error: {}", err)
    } 
}