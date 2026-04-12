
/*
 ? Coin Game
 * A continuacion se agrega el ejemplo con una complejidad de 0(2^N)
 */
pub fn game_winner(number_of_coins: i32, current_player: Option<String>) -> String {
    let current_player = current_player.unwrap_or(String::from("you"));
    let mut next_player = String::new();

    if number_of_coins <= 0 {
        return current_player;
    }

    if current_player == "you" {
        next_player = format!("them");
    } else if current_player == "them" {
        next_player = format!("you");
    }

    if game_winner(number_of_coins - 1, Some(next_player.to_string())) == current_player ||
    game_winner(number_of_coins - 2, Some(next_player.to_string())) == current_player {
        current_player
    } else {
        next_player
    }
}


// * A continuacion se agrega el ejemplo con una complejidad de 0(1)

pub fn game_winner_faster(number_of_coins: i32) -> String {
    if (number_of_coins - 1) % 3 == 0 {
        return format!("them");
    }

    format!("you")
}