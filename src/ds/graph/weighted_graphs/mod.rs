use std::{cell::RefCell, rc::Rc};

use crate::ds::graph::weighted_graphs::weighted_graph_vertex::{WeightedGraph, WeightedGraphType, WeightedGraphVertex, dijstra_shortest_path};

pub mod weighted_graph_vertex;

pub fn test_1_weighted_graph() {
    let dallas = Rc::new(RefCell::new(WeightedGraphVertex::new(String::from("Dallas"))));
    let toronto = Rc::new(RefCell::new(WeightedGraphVertex::new(String::from("Toronto"))));

    dallas.borrow_mut().add_adjacent_vertex(Rc::clone(&toronto), 138);
    toronto.borrow_mut().add_adjacent_vertex(Rc::clone(&dallas), 216);
}

pub fn test_2_weighted_graph() {
    let atlanta: WeightedGraphType<String, u32> = Rc::new(RefCell::new(WeightedGraphVertex::new(String::from("Atlanta"))));
    let boston: WeightedGraphType<String, u32> = Rc::new(RefCell::new(WeightedGraphVertex::new(String::from("Boston"))));
    let chicago: WeightedGraphType<String, u32> = Rc::new(RefCell::new(WeightedGraphVertex::new(String::from("Chicago"))));
    let denver: WeightedGraphType<String, u32> = Rc::new(RefCell::new(WeightedGraphVertex::new(String::from("Denver"))));
    let el_paso: WeightedGraphType<String, u32> = Rc::new(RefCell::new(WeightedGraphVertex::new(String::from("El Paso"))));

    atlanta.borrow_mut().add_adjacent_vertex(Rc::clone(&boston), 100);
    atlanta.borrow_mut().add_adjacent_vertex(Rc::clone(&denver), 160);

    boston.borrow_mut().add_adjacent_vertex(Rc::clone(&chicago), 120);
    boston.borrow_mut().add_adjacent_vertex(Rc::clone(&denver), 180);

    chicago.borrow_mut().add_adjacent_vertex(Rc::clone(&el_paso), 80);

    denver.borrow_mut().add_adjacent_vertex(Rc::clone(&chicago), 40);
    denver.borrow_mut().add_adjacent_vertex(Rc::clone(&el_paso), 140);

    el_paso.borrow_mut().add_adjacent_vertex(Rc::clone(&boston), 100);


    let mut index = 0;
    let cities = dijstra_shortest_path(atlanta, el_paso);
    for city in &cities  {
        if index != cities.len() {
            print!("{}", city);
        }
        if index != cities.len() - 1 {
            print!(" -> ");
        }
        
        index += 1;
    }
}