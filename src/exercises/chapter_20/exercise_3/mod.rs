/*
 * 3.- You’re working on some more stock-prediction software. The function
 * you’re writing accepts an array of predicted prices for a particular stock
 * over the course of time.
 * For example, this array of seven prices:
 * [10, 7, 5, 8, 11, 2, 6]
 * predicts that a given stock will have these prices over the next seven days.
 * (On Day 1, the stock will close at $10; on Day 2, the stock will close at
 * $7; and so on.)
 * 
 * Your function should calculate the greatest profit that could be made
 * from a single “buy” transaction followed by a single “sell” transaction.
 * In the previous example, the most money could be made if we bought the
 * stock when it was worth $5 and sold it when it was worth $11. This yields
 * a profit of $6 per share.
 * Note that we could make even more money if we buy and sell multiple
 * times, but for now, this function focuses on the most profit that could be
 * made from just one purchase followed by one sale.
 * Now, we could use nested loops to find the profit of every possible buyand-
 * sell combination. However, this would be O(N2) and too slow for our
 * hotshot trading platform. Your job is to optimize the code so that the
 * function clocks in at just O(N).
*/

// fn predicted_prices(prices_per_day: &Vec<i32>) -> i32 {
//     let mut buy_transaction = prices_per_day[0];
//     let mut sell_transaction = prices_per_day[1];
    
//     for i in 2..=prices_per_day.len() - 1 {
//         let current_price = prices_per_day[i];

//         if current_price > 0 && current_price < buy_transaction {
//             buy_transaction = current_price;
//         }

//         if prices_per_day[i + 1] > sell_transaction {
//             sell_transaction = prices_per_day[i + 1];
//         }

//         if buy_transaction > sell_transaction {
//             let flag = buy_transaction;
//             buy_transaction = sell_transaction;
//             sell_transaction = flag;
//         }
//     }


//     sell_transaction - buy_transaction
// }

fn predicted_prices(prices_per_day: &Vec<i32>) -> i32 {
    let mut min_buy = prices_per_day[0];
    let mut max_profit = 0;

    for &price in prices_per_day.iter().skip(1) {
        let potential_profit = price - min_buy;

        if potential_profit > max_profit {
            max_profit = potential_profit;
        }

        if price < min_buy {
            min_buy = price;
        }
    }

    max_profit
}

pub fn exercise_3() {
    let week_prices = vec![10, 7, 5, 8, 11, 2, 6];
    println!("\n---------- Ejercicio 3: ----------");
    println!("\nLos precios de la semana son: {:?}\nEl resultado es: {}", &week_prices, predicted_prices(&week_prices));
}