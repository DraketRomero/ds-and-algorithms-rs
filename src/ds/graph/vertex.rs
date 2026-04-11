use std::{cell::RefCell, collections::HashMap, fmt::Debug, hash::Hash, rc::Rc};

use crate::ds::queue::{Queue, QueueTrait};

type Elements<T> = Vec<Rc<RefCell<Vertex<T>>>>;
pub type NewElement<T> = Rc<RefCell<Vertex<T>>>;

pub trait Graph<T> {
    fn new(value: T) -> Self;
    fn add_adjacent_vertex(&mut self, vertex: NewElement<T>, myself: NewElement<T>);
    fn dfs_traverse(&self, vertex: &NewElement<T>, visited_vertices: &mut HashMap<T, bool>);
    fn dfs(
        &self,
        vertex: &NewElement<T>,
        searched_value: &T,
        visited_vertices: &mut HashMap<T, bool>,
    ) -> Option<NewElement<T>>;
    fn bfs_traverse(&self, startin_vertex: NewElement<T>);
    fn bfs(
        &self,
        startin_vertex: NewElement<T>,
        searched_value: T,
        visited_vertices: &mut HashMap<T, bool>,
    ) -> Option<NewElement<T>>;
}

#[derive(Debug)]
pub struct Vertex<T> {
    pub value: T,
    pub adjacent_vertices: Elements<T>,
}

impl<T: Eq + Hash + Clone + Debug> Graph<T> for Vertex<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            adjacent_vertices: Vec::new(),
        }
    }

    // * Version para un grafo dirigido
    // fn add_adjacent_vertex(&mut self, vertex: Rc<RefCell<Vertex<T>>>) {
    //     self.adjacent_vertices.push(vertex);
    // }

    // * Version para un grafo no dirigido
    fn add_adjacent_vertex(&mut self, vertex: NewElement<T>, myself: NewElement<T>) {
        if self
            .adjacent_vertices
            .iter()
            .any(|v| v.borrow().value == vertex.borrow().value)
        {
            return;
        }

        self.adjacent_vertices.push(Rc::clone(&vertex));

        vertex.borrow_mut().adjacent_vertices.push(myself);
    }

    /*
    ? Depth-First Search (DFS)
    * 1.- Start at any random vertex within the graph.

    * 2.- Add the current vertex to the hash table to
    * mark it as having been visited.

    * 3.- Iterate through the current vertex's adjacent
    * vertices.

    * 4.- For each adjacent vertex, if the adjacent
    * vertex has already been visited, ignore it.

    * 5.- If the adjacent vertex has not yet visited,
    * recursively perform depth-first search on that
    * vertex.
    */
    fn dfs_traverse(&self, vertex: &NewElement<T>, visited_vertices: &mut HashMap<T, bool>) {
        // * Mark vertex as visited by adding it to the hash table
        visited_vertices.insert(vertex.borrow().value.clone(), true);

        // * Print the vertex's value, so we can make sure our traversal really works
        println!("{:?}", vertex.borrow().value);

        // * Iterate through the current vertex's adjacent vertices:
        for adj_vrtx in &vertex.borrow().adjacent_vertices {
            // * Ignore an adjacent vertex if we've already visited it:
            if !visited_vertices.contains_key(&adj_vrtx.borrow().value) {
                self.dfs_traverse(adj_vrtx, visited_vertices);
            }
        }
    }

    /*
     * If we want to use depth-first search to actually search for a particular vertex,
     * we can use a modified version of the previous function:
     */
    fn dfs(
        &self,
        vertex: &NewElement<T>,
        searched_value: &T,
        visited_vertices: &mut HashMap<T, bool>,
    ) -> Option<NewElement<T>> {
        /*
         * Return the original vertex if it happens to
         * be the one we're searching for:
         */
        if vertex.borrow().value == *searched_value {
            return Some(Rc::clone(vertex));
        }

        visited_vertices.insert(vertex.borrow().value.clone(), true);

        for adj_vrtx in &vertex.borrow().adjacent_vertices {
            // * Ignore an adjacent vertex if we've already visited it:
            if !visited_vertices.contains_key(&adj_vrtx.borrow().value) {
                // self.dfs_traverse(adj_vrtx, visited_vertices);

                /*
                 * If the adjacent vertex is the vertex we're searching for, just
                 * return that vertex:
                 */
                if adj_vrtx.borrow().value == *searched_value {
                    return Some(Rc::clone(adj_vrtx));
                }

                /*
                 * Attempt to find the vertex we're searching for by recursively
                 * calling this method on the adjacent vertex:
                 */
                let vertex_were_searching_for =
                    self.dfs(adj_vrtx, searched_value, visited_vertices);

                /*
                 * If we were able to find the correct vertex using the above recursion,
                 * return the correct vertex:
                 */
                if let Some(vrx) = vertex_were_searching_for {
                    return Some(vrx);
                }
            }
        }

        None
    }

    /*
    ? Breadth-First Search (BFS)
    * 1.- Start at any vertex within the graph. We'll call this the "starting vertex".

    * 2.- Add the starting vertex to the hash table to mark it as having been visited.

    * 3.- Add the starting vertex to a queue.

    * 4.- Start a loop that runs while the queue isn't empty.

    * 5.- Within this loop, remove the first vertex from the queue.

    * 6.- Iterate over all the adjacent vertices of the current vertex.

    * 7.- If the adjacent vertex was already visited, ignore it.

    * 8.- If the adjacent vertex has not yet visited, mark it as visited by adding it
    * to the hash table, and add it to the queue.

    * 9.- Repeat this loop (starting from step 4) until the queue is empty.
    */
    fn bfs_traverse(&self, startin_vertex: NewElement<T>) {
        let mut queue: Queue<NewElement<T>> = Queue::new();
        let mut visited_vertices: HashMap<T, bool> = HashMap::new();

        visited_vertices.insert(startin_vertex.borrow().value.clone(), true);
        queue.enqueue(startin_vertex);

        // * While the queue is not empty:
        while let Some(current_vertex) = queue.dequeue() {
            // * Remove the first vertex off the queue and make it the current vertex:
            // let current_vertex = queue.dequeue().unwrap();

            // * Print the current vertex's value:
            println!("{:?}", current_vertex.borrow().value);

            // * Iterate over current vertex's adjacent vertices:
            for adj_vrtx in &current_vertex.borrow().adjacent_vertices {
                // * if we have not yet visited the adyacent vertex:
                if !visited_vertices.contains_key(&adj_vrtx.borrow().value) {
                    // * Mark the adjacent vertex as visited:
                    visited_vertices.insert(adj_vrtx.borrow().value.clone(), true);

                    // * Add the adjacent vertex to the queue:
                    queue.enqueue(Rc::clone(adj_vrtx));
                }
            }
        }
    }

    // ? Exercise
    fn bfs(
        &self,
        startin_vertex: NewElement<T>,
        searched_value: T,
        visited_vertices: &mut HashMap<T, bool>,
    ) -> Option<NewElement<T>> {
        if startin_vertex.borrow().value == searched_value {
            return Some(Rc::clone(&startin_vertex));
        }

        let mut queue: Queue<NewElement<T>> = Queue::new();

        visited_vertices.insert(startin_vertex.borrow().value.clone(), true);
        queue.enqueue(startin_vertex);

        while let Some(current_vertex) = queue.dequeue() {
            if current_vertex.borrow().value == searched_value {
                return Some(Rc::clone(&current_vertex));
            }

            for adj_vrtx in &current_vertex.borrow().adjacent_vertices {
                if !visited_vertices.contains_key(&adj_vrtx.borrow().value) {
                    visited_vertices.insert(adj_vrtx.borrow().value.clone(), true);
                    queue.enqueue(Rc::clone(adj_vrtx));
                }
            }
        }

        None
    }
}
