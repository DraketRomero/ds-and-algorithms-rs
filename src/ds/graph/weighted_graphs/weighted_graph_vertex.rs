use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

pub type WeightedGraphType<T, K> = Rc<RefCell<WeightedGraphVertex<T, K>>>;

// ✅ Wrapper para poder usar Rc<RefCell<...>> como clave en HashMap:
#[derive(Debug, Clone)]
pub struct RcKey<T, K>(pub Rc<RefCell<WeightedGraphVertex<T, K>>>);

impl<T: Hash, K> Hash for RcKey<T, K> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.borrow().value.hash(state);
    }
}

impl<T: PartialEq, K> PartialEq for RcKey<T, K> {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().value == other.0.borrow().value
    }
}

impl<T: Eq, K> Eq for RcKey<T, K> {}

pub trait WeightedGraph<T, K> {
    fn new(value: T) -> Self;
    fn add_adjacent_vertex(&mut self, vertex: WeightedGraphType<T, K>, weight: K);
}

#[derive(Debug)]
pub struct WeightedGraphVertex<T, K> {
    pub value: T,
    pub adjacent_vertices: HashMap<RcKey<T, K>, K>
}

impl<T: Eq + Hash + Clone, K> WeightedGraph<T, K> for WeightedGraphVertex<T, K> {
    fn new(value: T) -> Self {
        Self { value, adjacent_vertices: HashMap::new() }
    }

    fn add_adjacent_vertex(&mut self, vertex: WeightedGraphType<T, K>, weight: K) {
        self.adjacent_vertices.insert(RcKey(vertex), weight);
    }
}

/*
 ? Dijkstra's Algorithm Steps
 * 1.- We visit the starting vertex, making it our current vertex.
  
 * 2.- We check the prices from the current vertex to each of its adjacents vertices.
  
 * 3.- If the price to an adjacent vertex from the starting vertex is cheaper than the
 * price currently in cheapest_prices_table (or the adjacent vertex isn’t yet in the
 * cheapest_prices_table at all):
  
 *      a. We update the cheapest_prices_table to reflect this cheaper price.
 *      b. We update the cheapest_previous_stopover_vertex_table, making the adjacent vertex
 *      the key and the current vertex the value.
  
 * 4. We then visit whichever unvisited vertex has the cheapest price from the
 * starting vertex, making it the current vertex.
  
 * 5. We repeat the Steps 2 through 4 until we’ve visited every known vertex
 */
pub fn dijstra_shortest_path<T>(starting_city: WeightedGraphType<T, u32>, final_destination: WeightedGraphType<T, u32>) -> Vec<T>
where 
    T: Eq + Hash + Clone
{
    let mut cheapest_prices_table: HashMap<T, u32> = HashMap::new();
    let mut cheapest_previous_stopover_city_table: HashMap<T, T> = HashMap::new();

    /*
     * To keep our code simple, we'll use a basic array to keep track of
     * the known cities we haven't yet visited:
     */
    let mut unvisited_cities: Vec<WeightedGraphType<T, u32>> = Vec::new();

    /*
     * We keep track of the cities we've visited using a hash table.
     * We could have used an array, but since we'll be doing lookups,
     * a hash table is more efficient:
     */
    let mut visited_cities: HashMap<T, bool> = HashMap::new();

    /*
     * We add the starting city's name as the first key inside the
     * cheapest_prices_table. It has a value of 0, since it costs nothing
     * to get there:
     */
    cheapest_prices_table.insert(starting_city.borrow().value.clone(), 0);
    let mut current_city = Rc::clone(&starting_city);

    /*
     * This loop is the core of the algorithm. It runs as long as we can
     * visit a city that we haven't visited yet:
     */
    while !unvisited_cities.is_empty() || !visited_cities.contains_key(&current_city.borrow().value) {
        /*
         * We add the current city's name to the visited_cities hash to record
         * that we've officially visited it. We also remove it from the list
         * of unvisited cities:
         */
        visited_cities.insert(current_city.borrow().value.clone(), true);
        unvisited_cities.retain(|city| city.borrow().value != current_city.borrow().value);

        // * We iterate over each of the current_city's adjacent cities:
        for (adjacent_city, price) in &current_city.borrow().adjacent_vertices {
            let adjacent_city = &adjacent_city.0;

            /*
             * If we've discover a new city,
             * we add it to the list of unvisited_cities:
             */
            if !visited_cities.contains_key(&adjacent_city.borrow().value) && !unvisited_cities.iter().any(|c| c.borrow().value == adjacent_city.borrow().value) {
                unvisited_cities.push(Rc::clone(adjacent_city));
            }

            /*
             * We calculate the price of getting from the STARTING city to the
             * ADJACENT city using the CURRENT city as the second-to-last stop:
             */
            let price_through_current_city = cheapest_prices_table.get(&current_city.borrow().value).unwrap_or(&0) + price;

            /*
             * If the price from the STARTING city to the ADJACENT is
             * the cheapest one we've found so far...
             */
            if !cheapest_prices_table.contains_key(&adjacent_city.borrow().value) || price_through_current_city < *cheapest_prices_table.get(&adjacent_city.borrow().value).unwrap() {

                // * ....we update our two tables:
                cheapest_prices_table.insert(adjacent_city.borrow().value.clone(), price_through_current_city);
                cheapest_previous_stopover_city_table.insert(adjacent_city.borrow().value.clone(), current_city.borrow().value.clone());
            }
        }

        /*
         * We visit our next unvisited city. We choose the one that is cheapest
         * to get to from the STARTING city:
         */
        let unvisited_cheapest_city = unvisited_cities.iter().min_by_key(|city| cheapest_prices_table.get(&city.borrow().value).unwrap_or(&u32::MAX)).cloned();
        if let Some(next_city) = unvisited_cheapest_city {
            current_city = next_city
        } else {
            break;
        }
    }

    /*
     * We have completed the core algorithm. At this point, the
     * cheapest_prices_table contains all the cheapest prices to get to each
     * city from the starting city. However, to calcute the precise path 
     * to take from our starting city to our final destination,
     * we need to move on.
     * 
     * We'll build the shortest path using a simple array:
     */
    let mut shortest_path: Vec<T> = Vec::new();

    /*
     * To construct the shortest path, we need to work backwards from our
     * final destination. So, we begin with the final destination as our 
     * current_city_name:
     */
    let mut current_city_name = final_destination.borrow().value.clone();

    // * We loop until we reach our starting city 
    while current_city_name.clone() != starting_city.borrow().value {
        // * We add each current_city_name we encounter to the shortest path array:
        shortest_path.push(current_city_name.clone());

        /*
         * We use the cheapest_previous_stopover_city_table to follow each city
         * to its previous stopover city:
         */
        if let Some(city) = cheapest_previous_stopover_city_table.get(&current_city_name) {
            current_city_name = city.clone();
        }
    }

    // * We cap things off by adding the starting city to the shortest path:
    shortest_path.push(starting_city.borrow().value.clone());

    // * We reverse the output so we can see the path from beginning to end:
    shortest_path.reverse();
    shortest_path
}