/*
 * Recursive Category: Repeatedly Execute
 * Este ejemplo representa y explica la categoria definida como ejecucion repetida, donde la solucion al problema "Duplicar cada elemento del array proporcionado como parametro" es implementada y explicada.
 *
 * Recursive trick: Passing Extra Parameters
 * Durante el desarrollo del ejemplo, se explica que un truco para cuando no se pueden pasar parametros extras a una funcion por el hecho del propio diseño de la funcion recursiva al inicio de la primera llamda, es usar parametros extras o invisibles, definiendo su valor a 0 en su definicion inicial.
 */

pub fn double_value_in_array(numbers: &mut Vec<usize>, index: Option<usize>) {
    let n = index.unwrap_or(0);

    // * Case base: When index goes past the end of the array
    if n >= numbers.len() {
        return;
    }

    numbers[n] *= 2;

    double_value_in_array(numbers, Some(n + 1));
}

/*
 * Recursive Category: Calculations
 * Otro de los ejemplos mostrados es cuando se necesita calcular algo, por ejemplo, el factorial de un numero. Aqui se muestra la solucion usando ciclos...
 */

pub fn factorial(number: i32) -> i32 {
    if number == 1 {
        return 1;
    }

    number * factorial(number - 1)
}

/*
 * ... y usando recursividad.
 * Existen 2 enfoques a la resolucion de este tipo de problemas, uno es "bottom to up" y otro "top-down", esto basado en el subproblema del problema. Ambas son estrategias de resolucion de problemas.
 */
pub fn factorial_with_recursion(n: usize, i: Option<usize>, product: Option<usize>) -> usize {
    let i_val = i.unwrap_or(0);
    let product_val = product.unwrap_or(0);

    if i_val > n {
        return product_val;
    }

    factorial_with_recursion(n, Some(i_val + 1), Some(product_val * i_val))
}

/*
 * Top-down recursion
 * Es una estrategia mental para la resolucion de problemas de recursividad, donde se enfoca uno en resolver el problema en el siguiente proceso:
 * 1.- Imagina que la funcion ya fue implementada por alguien mas.
 * 2.- Identifica el subproblema del problema
 * 3.- Mira que pasa cuando llamas a la funcion en el subproblema y parte de ahi.
 *
 * Ejemplo 1: Array sum - Sumar todos los valores de un array.
 */

pub fn array_sum(numbers: &[usize]) -> usize {
    // * Base case - Si el array esta vacio o si tiene 1 solo elemento.
    if numbers.is_empty() {
        return 0;
    }

    return numbers[0] + array_sum(&numbers[1..]);
}

// * Ejemplo 2: String reversal - Invertir los caracteres de una palabra dada.
pub fn reverse(text: &str) -> String {
    // * Base case - Cuando el string esta vacio.
    if text.is_empty() {
        return String::new();
    }

    return reverse(&text[1..]) + &text[..1];
}

// * Ejemplo 3: Counting X - Calcula la cantidad total de X en un texto dado.
pub fn count_x(text: &str) -> usize {
    if text.len() == 0 {
        return 0;
    }

    let mut chars = text.chars();
    let first = chars.next().unwrap();

    if first == 'x' {
        return 1 + count_x(chars.as_str());
    }

    count_x(&text[1..])
}

// * Ejemplo 4: Staircase problem - Calcula cuantos escalones puede subir una persona siguiendo un patron, ya sea de 1 en 1, 2 en 2, mixto, etc.
pub fn number_of_paths(paths: usize) -> usize {
    if paths == 0 {
        return 0;
    } else if paths == 1 || paths == 0 {
        return 1;
    }

    number_of_paths(paths - 1) + number_of_paths(paths - 2) + number_of_paths(paths - 3)
}

/*
 * Ejemplo 5: Generacion de anagramas - Un anagrama es la convinacion de todas las letras posibles de un string dado, P/E: Los anagramas de "abc" son:
 * ["abc", "acb", "bac", "bca", "cab", "cba"]
 */
pub fn anagrams_of(text: &str) -> Vec<String> {
    // * Crea un array para almacenar todos los anagramas
    let mut anagrams: Vec<String> = Vec::new();

    // * Base case: Si el string es solo un caracter.
    let mut chars = text.chars();
    if chars.clone().count() == 1 {
        return vec![text.to_string()];
    }

    /*
     * Encuentra todos los anagramas del substring desde del segundo caracter hasta el final.
     * Por ejemplo, si el string es "abcd", el substrng es "bcd", por lo que encontraremos
     * todos los anagramas de "bcd".
     */
    let first_char = chars.next().unwrap();
    let rest = chars.as_str();

    let substring_anagrams = anagrams_of(rest);

    for substring in substring_anagrams {
        for index in 0..=substring.len() {
            let mut copy = substring.clone();
            copy.insert(index, first_char);
            anagrams.push(copy);
        }
    }

    anagrams
}
