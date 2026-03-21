pub struct Algorithms<T> {
    pub attr_reader: Vec<T>,
}

impl<T> Algorithms<T> {
    pub fn new(arr: Vec<T>) -> Self {
        Self { attr_reader: arr }
    }
}

impl<T> Algorithms<T> {
    pub fn linear_search(&self, search_value: &T) -> Option<usize>
    where
        T: PartialEq + PartialOrd,
    {
        // * Iteramos sobre cada elemento del array
        for index in 0..self.attr_reader.len() {
            let element = &self.attr_reader[index];

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

    pub fn binary_search(&self, search_value: &T) -> Option<usize>
    where
        T: PartialEq + PartialOrd,
    {
        // * Primero se establecen las variables para definir los valores mas bajos y mas altos donde el valor buscado puede estar. Para empezar, el valor mas bajo es el valor mas bajo del array, mientras que el mas alto es el ultimo valor.
        let mut lower_bound: usize = 0;
        let mut upper_bound: usize = self.attr_reader.len() - 1;

        // * Comenzamos un ciclo while, en el cual se mantiene inspeccionando el valor mas al medio entre las variables mayor y menor
        while lower_bound <= upper_bound {
            // * Encontramos el punto medio entre las variaable de valor mas alto y la del mas bajo
            let midpoint = (upper_bound + lower_bound) / 2;

            // * Inspeccionamos el valor al punto medio.
            let value_at_midpoint = &self.attr_reader[midpoint];

            // * Si el valor del punto medio es el que estamos buscando, terminamos el ciclo. De lo contrario, cambiamos el valor mas bajo o mas alto basado en si necesitamos movernos a la derecha o la izquierda.
            if search_value == value_at_midpoint {
                return Some(midpoint);
            } else if search_value < value_at_midpoint {
                if midpoint == 0 {
                    return None;
                }
                upper_bound = midpoint - 1;
            } else if search_value > value_at_midpoint {
                lower_bound = midpoint + 1;
            }
        }

        // * Si hemos reducido las variables hasta que han alcanzado una a la otra, entonces significa que el valor que estamos buscando no lo contiene el array.
        return None;
    }

    pub fn bubble_sort(&self) -> Vec<T>
    where
        T: PartialOrd + Clone,
    {
        let mut new_vector: Vec<T> = self.attr_reader.clone();
        let mut unsorted_until_index = new_vector.len() - 1;
        let mut sorted = false;

        while !sorted {
            sorted = true;

            // * Usar unsorted_until_index en lugar de new_vector.len()
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

    pub fn selection_sort(&mut self) -> Vec<T>
    where
        T: PartialOrd + Copy,
    {
        let mut new_vec = self.attr_reader.clone();
        for i in 0..new_vec.len() - 1 {
            let mut lowest_number_index = i;
            for j in (i + 1)..new_vec.len() {
                if new_vec[j] < new_vec[lowest_number_index] {
                    lowest_number_index = j;
                }
            }

            if lowest_number_index != i {
                let temp = new_vec[i];
                new_vec[i] = new_vec[lowest_number_index];
                new_vec[lowest_number_index] = temp;
            }
        }

        return new_vec.to_owned();
    }

    // * Version implementada identica a la del libro
    pub fn insertion_sort(&self) -> Vec<T>
    where
        T: PartialEq + PartialOrd + Clone,
    {
        let mut new_vec = self.attr_reader.clone();
        for index in 1..new_vec.len() {
            let temp_value: T = new_vec[index].clone();
            let mut position: i64 = (index - 1) as i64;

            while position >= 0 {
                if new_vec[position as usize] > temp_value {
                    new_vec[(position + 1) as usize] = new_vec[position as usize].clone();
                    position -= 1;
                } else {
                    break;
                }
            }
            new_vec[(position + 1) as usize] = temp_value;
        }

        new_vec
    }

    // * Version corregida
    // pub fn insertion_sort(vector: Vec<T>) -> Vec<T>
    // where
    //     T: PartialEq + PartialOrd + Clone,
    // {
    //     let mut new_vec = vector.clone();
    //     for index in 1..new_vec.len() {
    //         let temp_value: T = new_vec[index].clone();
    //         let mut position = index;

    //         while position > 0 && new_vec[position - 1] > temp_value {
    //             new_vec[position] = new_vec[position - 1].clone();
    //             position -= 1;
    //         }

    //         new_vec[position] = temp_value;
    //     }

    //     new_vec.to_owned()
    // }

    pub fn partiton(&mut self, mut left_pointer: usize, mut right_pointer: usize) -> usize
    where
        T: PartialEq + PartialOrd + Clone,
    {
        /*
         * We always choose the right-most element as the pivot.
         * We keep the index of the pivot for later use:
         */
        let pivot_index = right_pointer;

        // * We grab the pivot value itself:
        let pivot = self.attr_reader[pivot_index].clone();

        // * We start the right pointer immediately to the left of the pivot
        right_pointer -= 1;

        loop {
            /*
             * Move the left pointer to the right as long as it
             * points to value that is less than the pivot:
             */
            while self.attr_reader[left_pointer] < pivot {
                left_pointer += 1;
            }

            /*
             * Move the right pointer to the left as long as it
             * points to a value that is greater than the pivot:
             */
            while self.attr_reader[right_pointer] > pivot {
                if right_pointer == 0 {
                    break;
                }
                right_pointer -= 1;
            }

            /*
             * We've now reached the point where we've stopped
             * moving both the left and right pointers.
             *
             * We check whether the left pointer has reached
             * or gone beyond the right pointer. If it has,
             * we break out of the loop so we can swap the pivot later
             * on in our code:
             */
            if left_pointer >= right_pointer {
                break;
            }
            /*
             * If the left pointer is still to the left of the right
             * pointer, we swap the values of the left and right pointers:
             */
            else {
                self.attr_reader.swap(left_pointer, right_pointer);

                /*
                 * We move the left pointer over to the right, gearing up
                 * for the next round of left and right pointer movements:
                 */
                left_pointer += 1;
            }
        }

        /*
         * As the final step of the partition, we swap the value
         * of the left pointer with the pivot:
         */
        self.attr_reader.swap(left_pointer, pivot_index);

        // * We return the left_pointer for the sake of the quicksort method
        left_pointer
    }

    pub fn quicksort(&mut self, left_index: usize, right_index: usize)
    where
        T: PartialEq + PartialOrd + Clone,
    {
        // * Base case: The subarray has 0 or 1 elements:
        if right_index <= left_index {
            return;
        }

        // * Partition the range of elements and grab the index of the pivot:
        let pivot_index = self.partiton(left_index, right_index);

        // * Se agrega el condicion en Rust por el problema de underflow.
        if pivot_index > 0 {
            /*
             * Recursively call this quicksort method whatever
             * is to the left of the pivot:
             */
            self.quicksort(left_index, pivot_index - 1);
        }

        /*
         * Recursively call this quicksort method whatever
         * is to the right of the pivot:
         */
        self.quicksort(pivot_index + 1, right_index)
    }

    pub fn quickselect(
        &mut self,
        kth_lowest_value: usize,
        left_index: usize,
        right_index: usize,
    ) -> T
    where
        T: PartialOrd + Clone,
    {
        /*
         * If we reach a base case - that is, that the subarray has one cell,
         * we know we've found the value we're looking for:
         */
        if right_index <= left_index {
            return self.attr_reader[left_index].clone();
        }

        // * Partition the array and grab the index of the pivot:
        let pivot_index = self.partiton(left_index, right_index);

        // * If what we're looking for is to the left of the pivot:
        if kth_lowest_value < pivot_index {
            /*
             * Recursively perform quickselect on the subarray to
             * the left of the pivot
             */
            return self.quickselect(kth_lowest_value, left_index, pivot_index - 1);
        }
        // * If what we're looking for is to the right of the pivot
        else if kth_lowest_value > pivot_index {
            /*
             * Recursively perform quickselect on the subarray to
             * the right of the pivot
             */
            return self.quickselect(kth_lowest_value, pivot_index + 1, right_index);
        }

        /*
         * If the kth_lowest_value == pivot_index, after the partition, the pivot position is in the same spot
         * as the kth lowest value, we've found the value we're looking for
         */
        return self.attr_reader[pivot_index].clone();
    }
}
