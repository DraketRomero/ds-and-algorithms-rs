pub enum Element {
    Numero(i32),
    Vec(Vec<Element>)
}

pub fn print_numbers_from_arrays(element: &[Element]) {
    for ele in element {
        match ele {
            Element::Numero(n) => {
                println!("{}", n);
            },
            Element::Vec(vec) => {
                print_numbers_from_arrays(vec);
            }
        }
    }
}