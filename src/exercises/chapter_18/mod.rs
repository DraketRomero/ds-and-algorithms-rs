/*
* Respuestas al ejercicio 1:
*
* 1.- Pins
* 2.- Needles
* 3.- Nail polish
* 4.- Hammer

* Respuestas al ejercicio 2:
*
* A
* B, E, J, F, O
* C, G, K
* D, H, L, M, I, N, P

* Respuestas al ejercicio 3:
*
* A
* B, C, D
* E, F,
* G
* H, I
* J
* K
* L
* M
* N
* O
* P
*
* Respuesta al ejercicio 4:
*/
// fn bfs(
//     &self,
//     startin_vertex: NewElement<T>,
//     searched_value: T,
//     visited_vertices: &mut HashMap<T, bool>,
// ) -> Option<NewElement<T>> {
//     if startin_vertex.borrow().value == searched_value {
//         return Some(Rc::clone(&startin_vertex));
//     }

//     let mut queue: Queue<NewElement<T>> = Queue::new();

//     visited_vertices.insert(startin_vertex.borrow().value.clone(), true);
//     queue.enqueue(startin_vertex);

//     while let Some(current_vertex) = queue.dequeue() {
//         if current_vertex.borrow().value == searched_value {
//             return Some(Rc::clone(&current_vertex));
//         }

//         for adj_vrtx in &current_vertex.borrow().adjacent_vertices {
//             if !visited_vertices.contains_key(&adj_vrtx.borrow().value) {
//                 visited_vertices.insert(adj_vrtx.borrow().value.clone(), true);
//                 queue.enqueue(Rc::clone(adj_vrtx));
//             }
//         }
//     }

//     None
// }

use std::{collections::HashMap, hash::Hash, rc::Rc};

use crate::ds::{graph::vertex::NewElement, queue::{Queue, QueueTrait}};

// * Respuesta al ejercicio 5:
fn find_shortest_path<T>(first_vertex: NewElement<T>, second_vertex: NewElement<T>, visited_vertecies: &mut HashMap<T, bool>) -> Option<Vec<T>> 
where
    T: Eq + Hash + Clone,
{

    let mut queue: Queue<NewElement<T>> = Queue::new();

    /*
     * As in Dijkstra' algorithm, we keep trak in a table of each vertex
     * immediatly preceding vertex.
     */
    let mut previous_vertex_table = HashMap::new();

    // * We employ breath-first search:
    visited_vertecies.insert(first_vertex.borrow().value.clone(), true);
    queue.enqueue(Rc::clone(&first_vertex));

    while let Some(current_vertex) = queue.dequeue() {
        for vrtx in &current_vertex.borrow().adjacent_vertices {
            if !visited_vertecies.contains_key(&vrtx.borrow().value) {
                visited_vertecies.insert(vrtx.borrow().value.clone(), true);
                queue.enqueue(Rc::clone(vrtx));

                /*
                 * We store in the previous_vertex table the adjacent vertex
                 * as the key, and the current_vertex as the value. This
                 * indicates that the current_vertex is the immediatly 
                 * preceding vertex that leads to the adjacent_vertex.
                 */
                previous_vertex_table.insert(vrtx.borrow().value.clone(), current_vertex.borrow().value.clone());
            }
        }
    }

    /*
     * As in Dijkstra' algorithm,we work backwards through the
     * previous_vertex_table to build the shortest path:
     */
    let mut shortest_path: Vec<T> = Vec::new();
    let mut current_vertex_value = second_vertex.borrow().value.clone();

    while current_vertex_value != first_vertex.borrow().value.clone() {
        shortest_path.push(current_vertex_value.clone());
        if let Some(prev) = previous_vertex_table.get(&current_vertex_value) {
            current_vertex_value = prev.clone();
        } else {
            break;
        }
    }

    shortest_path.push(first_vertex.borrow().value.clone());
    shortest_path.reverse();
    
    return Some(shortest_path);
}