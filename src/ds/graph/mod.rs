pub mod vertex;
pub mod weighted_graphs;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use vertex::{Graph, Vertex};

pub fn test_vertices() {
    // let angie = Rc::new(RefCell::new(Vertex::new(String::from("Angeles"))));
    // let diego = Rc::new(RefCell::new(Vertex::new(String::from("Diego"))));
    // let rousel = Rc::new(RefCell::new(Vertex::new(String::from("Rousel"))));

    // angie
    //     .borrow_mut()
    //     .add_adjacent_vertex(Rc::clone(&diego), Rc::clone(&angie));
    // angie
    //     .borrow_mut()
    //     .add_adjacent_vertex(Rc::clone(&rousel), Rc::clone(&angie));

    // // * Verificar las conexiones:
    // println!("Vertices adyacentes a {}:", angie.borrow().value);
    // for v in &angie.borrow().adjacent_vertices {
    //     println!("  - {}", v.borrow().value);
    // }

    // println!("Vertices adyacentes a {}:", diego.borrow().value);
    // for v in &diego.borrow().adjacent_vertices {
    //     println!("  - {}", v.borrow().value);
    // }

    let alice = Rc::new(RefCell::new(Vertex::new(String::from("Alice"))));
    let bob = Rc::new(RefCell::new(Vertex::new(String::from("Bob"))));
    let candy = Rc::new(RefCell::new(Vertex::new(String::from("Candy"))));
    let derek = Rc::new(RefCell::new(Vertex::new(String::from("Derek"))));
    let elaine = Rc::new(RefCell::new(Vertex::new(String::from("Elaine"))));
    let fred = Rc::new(RefCell::new(Vertex::new(String::from("Fred"))));
    let helen = Rc::new(RefCell::new(Vertex::new(String::from("Helen"))));
    let gina = Rc::new(RefCell::new(Vertex::new(String::from("Gina"))));
    let irena = Rc::new(RefCell::new(Vertex::new(String::from("Irena"))));

    // * Vecinos de Alice
    alice
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&bob), Rc::clone(&alice));
    alice
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&candy), Rc::clone(&alice));
    alice
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&derek), Rc::clone(&alice));
    alice
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&elaine), Rc::clone(&alice));


    // * Vecinos de Bob
    bob
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&fred), Rc::clone(&bob));

    // * Vecinos de Fred
    fred
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&helen), Rc::clone(&fred));

    // * Vecinos de Helen
    helen
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&fred), Rc::clone(&helen));
    helen
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&candy), Rc::clone(&helen));

    // * Vecinos de Candy
    candy
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&helen), Rc::clone(&candy));
    candy
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&alice), Rc::clone(&candy));

    // * Vecinos de Derek
    derek
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&alice), Rc::clone(&derek));
    derek
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&elaine), Rc::clone(&derek));
    derek
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&gina), Rc::clone(&derek));

    // * Vecinos de Elaine
    elaine
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&alice), Rc::clone(&elaine));
    elaine
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&derek), Rc::clone(&elaine));

    // * Vecinos de Gina
    gina
    .borrow_mut()
    .add_adjacent_vertex(Rc::clone(&derek), Rc::clone(&gina));
    gina
        .borrow_mut()
        .add_adjacent_vertex(Rc::clone(&irena), Rc::clone(&gina));

    // * Vecinos de Irena
    irena
    .borrow_mut()
    .add_adjacent_vertex(Rc::clone(&gina), Rc::clone(&irena));

    // * Prueba de DFS traverse
    println!("Prueba de DFS traverse");
    let mut visited: HashMap<String, bool> = HashMap::new();
    alice.borrow().dfs_traverse(&alice, &mut visited);

    // * Prueba de DFS 
    println!("Prueba de DFS");
    let mut visited: HashMap<String, bool> = HashMap::new();
    let result = alice.borrow().dfs(&alice, &String::from("Hele"), &mut visited);

    match result {
        Some(value) => println!("Encontrado! {:?}", value.borrow().value),
        None => println!("No esta :c")
    };

    // * Prueba de BFS traverse
    println!("\n Prueba de BFS traverse");
    let starting_vertex = Rc::new(RefCell::new(Vertex::new(String::from("Helen"))));
    alice.borrow().bfs_traverse(starting_vertex);
}
