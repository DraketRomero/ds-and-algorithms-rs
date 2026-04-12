pub fn max_sum(array: Vec<i8>) -> i8 {
    let mut current_sum = 0;
    let mut greatest_sum = 0;

    for i in 0..array.len() {
        let num = array[i];

        // * If current sum is a negative number, reset current sum to zero:
        if current_sum + num < 0 {
            current_sum = 0;
        } else {
            current_sum += num;

            /*
             * Greedily assume the current sum is the greates sum
             * if it's the greatest sum we've encountered so far:
             */
            if current_sum > greatest_sum {
                greatest_sum = current_sum
            }
        }
    }

    return greatest_sum;
}