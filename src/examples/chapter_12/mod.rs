use std::collections::HashMap;

/*
 * Unnecessary Recursive Calls - Este ejemplo muestra el error mas comun en recursividad, los llamados Subproblemas superpuestos (Overlapping Subproblems).
 *
 * Este ejercicio obtiene el numero mas grande dentro de un array de numeros, ejecutando la misma con un tiempo de complejidad de o(2^n).
 */
pub fn max_2n(numbers: &Vec<usize>) -> Option<usize> {
    if numbers.len() == 0 {
        return None;
    } else if numbers.len() == 1 {
        return Some(numbers[0]);
    }

    if numbers[0] > max_2n(&numbers[1..].to_vec()).unwrap() {
        return Some(numbers[0]);
    }

    max_2n(&numbers[1..].to_vec())
}

// * La solucion aplicando un cambio en el codigo que permite ejecutar la funcion con un tiempo de complejidad de O(N).
pub fn max_n(numbers: &Vec<usize>) -> Option<usize> {
    if numbers.len() == 0 {
        return None;
    } else if numbers.len() == 1 {
        return Some(numbers[0]);
    }

    let max_of_remainder = max_n(&numbers[1..].to_vec()).unwrap_or(0);

    if numbers[0] > max_of_remainder {
        return Some(numbers[0]);
    } else {
        Some(max_of_remainder)
    }
}

/*
 * Otro ejemplo de Subproblemas superpuestos (Overlapping Subproblems) es el de la secuencia de Fibonacci, se ejecuta 2 veces la llamada recursiva de la funcion.
 */
pub fn fib_rec(cant: usize) -> usize {
    if cant == 0 || cant == 1 {
        return cant;
    }

    fib_rec(cant - 2) + fib_rec(cant - 1)
}

// * Para este tipo de problemas la solucion es la programacion dinamica, existen dos maneras de aplicarlo:

// * 1.- Aplicando algo llamado memoization:
pub fn fib_memo(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if n == 0 || n == 1 {
        return n;
    }

    // * Check the hash table (called memo) to see whether fib(n) was already computed or not:
    if !memo.contains_key(&n) {
        /*
         * If n is not in memo, compute fib(n) with recursion
         * and then store the result in the hash table,
         * but we calculate the result before to avoid borrowing.
         */
        let result = fib_memo(n - 2, memo) + fib_memo(n - 1, memo);
        memo.insert(n, result);
    }

    /*
     * By now, fib(n)'s result is certainly in memo, (Perhaps
     * it was there before, or perhaps we just stored it there
     * in the previous line of code. But it's certainly there now.)
     * So let's return it:
     */
    return memo[&n];
}

// * 2.- Bottom-up
pub fn fib_bot(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    /*
     * a and b start with the first two numbers in the 
     * series, respectively:
     */
    let mut a = 0;
    let mut b = 1;
    let mut temp;

    // * Loop from 1 until n:
    for _ in 1..n {
        /*
         * a and b each move up to the next numbers in the series.
         * Namely, b becomes the sum of b + a, and a becomes what b used to be.
         * We utilize a temporary variable to make these changes:
         */
        temp = a;
        a = b;
        b = temp + a;
    }

    return b;
}
