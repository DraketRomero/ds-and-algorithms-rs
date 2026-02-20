pub struct Algorithms;

impl Algorithms {
    pub fn new() -> Self {
        Self
    }
}

impl Algorithms {
    pub fn linear_search<T>(vector: &Vec<T>, search_value: &T) -> Option<usize>
    where
        T: PartialEq + PartialOrd,
    {
        // * Iteramos sobre cada elemento del array
        for index in 0..vector.len() {
            let element = &vector[index];

            // * Si encontramos el valor que estamos buscando, retornamos su indice.
            if element == search_value {
                return Some(index);
            }
            // * Si alcanzamos un elemento que es mayor que el valor que estamos buscando, finalizamos la busqueda de forma temprana y retornamos None.
            else if element > search_value {
                return None;
            }
        }

        // * Si no se se encuentra el valor, retornamos None
        None
    }

    pub fn binary_search<T>(vector: &Vec<T>, search_value: &T) -> Option<usize>
    where
        T: PartialEq + PartialOrd,
    {
        // * Primero se establecen las variables para definir los valores mas bajos y mas altos donde el valor buscado puede estar. Para empezar, el valor mas bajo es el valor mas bajo del array, mientras que el mas alto es el ultimo valor.
        let mut lower_bound: usize = 0;
        let mut upper_bound: usize = vector.len() - 1;

        // * Comenzamos un ciclo while, en el cual se mantiene inspeccionando el valor mas al medio entre las variables mayor y menor
        while lower_bound <= upper_bound {
            // * Encontramos el punto medio entre las variaable de valor mas alto y la del mas bajo
            let midpoint = (upper_bound + lower_bound) / 2;

            // * Inspeccionamos el valor al punto medio.
            let value_at_midpoint = &vector[midpoint];

            // * Si el valor del punto medio es el que estamos buscando, terminamos el ciclo. De lo contrario, cambiamos el valor mas bajo o mas alto basado en si necesitamos movernos a la derecha o la izquierda.
            if search_value == value_at_midpoint {
                return Some(midpoint);
            } else if search_value < value_at_midpoint {
                upper_bound = midpoint - 1;
            } else if search_value > value_at_midpoint {
                lower_bound = midpoint + 1;
            }
        }

        // * Si hemos reducido las variables hasta que han alcanzado una a la otra, entonces significa que el valor que estamos buscando no lo contiene el array.
        return None;
    }

    pub fn bubble_sort<T>(vector: &Vec<T>) -> Vec<T>
    where
        T: PartialOrd + Clone,
    {
        let mut new_vector: Vec<T> = vector.clone();
        let mut unsorted_until_index = new_vector.len() - 1;
        let mut sorted = false;

        while !sorted {
            sorted = true;

            // Usar unsorted_until_index en lugar de new_vector.len()
            for i in 0..unsorted_until_index {
                // * Si se detecta que hay un valor fuera de su lugar
                if new_vector[i] > new_vector[i + 1] {
                    // * Cambiamos la variable sorted a false.
                    sorted = false;

                    let resp = new_vector[i].clone();
                    new_vector[i] = new_vector[i + 1].clone();
                    new_vector[i + 1] = resp;

                    // ? Se puede usar el metodo swap en lugar de asignaciones manuales
                    // new_vector.swap(i, i + 1);
                }
            }

            unsorted_until_index -= 1;
        }

        return new_vector;
    }

    pub fn selection_sort<T>(vector: &mut Vec<T>) -> &mut Vec<T>
    where
        T: PartialOrd + Copy,
    {
        for i in 0..vector.len() - 1 {
            let mut lowest_number_index = i;
            for j in (i + 1)..vector.len() {
                if vector[j] < vector[lowest_number_index] {
                    lowest_number_index = j;
                }
            }

            if lowest_number_index != i {
                let temp = vector[i];
                vector[i] = vector[lowest_number_index];
                vector[lowest_number_index] = temp;
            }
        }

        return vector;
    }
}
