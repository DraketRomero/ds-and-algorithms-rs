
/*
 * 1.- You’re working on software that analyzes sports players. Following are two arrays of players of different sports:
  basketball_players = [
    {first_name: "Jill", last_name: "Huang", team: "Gators"},
    {first_name: "Janko", last_name: "Barton", team: "Sharks"},
    {first_name: "Wanda", last_name: "Vakulskas", team: "Sharks"},
    {first_name: "Jill", last_name: "Moloney", team: "Gators"},
    {first_name: "Luuk", last_name: "Watkins", team: "Gators"}
    ]

    football_players = [
    {first_name: "Hanzla", last_name: "Radosti", team: "32ers"},
    {first_name: "Tina", last_name: "Watkins", team: "Barleycorns"},
    {first_name: "Alex", last_name: "Patel", team: "32ers"},
    {first_name: "Jill", last_name: "Huang", team: "Barleycorns"},
    {first_name: "Wanda", last_name: "Vakulskas", team: "Barleycorns"}
    ]

 * If you look carefully, you’ll see that there are some players who participate
 * in more than one kind of sport. Jill Huang and Wanda Vakulskas play
 * both basketball and football.
 * You are to write a function that accepts two arrays of players and returns
 * an array of the players who play in both sports. In this case, that would be:
 * ["Jill Huang", "Wanda Vakulskas"]
 * While there are players who share first names and players who share last
 * names, we can assume there’s only one person who has a particular full
 * name (meaning first and last name).
 * We can use a nested-loops approach, comparing each player from one
 * array against each player from the other array, but this would have a
 * runtime of O(N * M). Your job is to optimize the function so that it can
 * run in just O(N + M).
 */

pub mod players;

use std::collections::HashMap;
use players::Players;

fn sports_players(basketball_players: Vec<Players>, football_players: Vec<Players>) -> Vec<String> {
    let mut players_hm: HashMap<String, usize> = HashMap::new();
    let mut players_found: Vec<String> = Vec::new();

    for player in basketball_players {
        let full_name = format!("{} {}", player.first_name, player.last_name);
        *players_hm.entry(full_name).or_insert(0) += 1;
    } 

    for player in football_players {
        let full_name = format!("{} {}", player.first_name, player.last_name);
        if players_hm.contains_key(&full_name) {
            players_found.push(full_name);
        }
    }

    players_found
}

pub fn exercise_1() {    
    // * Basketball Players
    let mut jill = Players::new();
    jill.new_player(String::from("Jill"), String::from("Huang"), String::from("Gators"));
    
    let mut janko = Players::new();
    janko.new_player(String::from("Janko"), String::from("Barton"), String::from("Sharks"));
    
    let mut wanda = Players::new();
    wanda.new_player(String::from("Wanda"), String::from("Vakulskas"), String::from("Sharks"));
    
    let mut jill_2 = Players::new();
    jill_2.new_player(String::from("Jill"), String::from("Moloney"), String::from("Gators"));
    
    let mut luuk = Players::new();
    luuk.new_player(String::from("Luuk"), String::from("Watkins"), String::from("Gators"));
    
    let basketball_players: Vec<Players> = Vec::from([jill, janko, wanda, jill_2, luuk]);

    // * Football Players
    let mut hanzla = Players::new();
    hanzla.new_player(String::from("Hanzla"), String::from("Radosti"), String::from("32ers"));

    let mut tina = Players::new();
    tina.new_player(String::from("Tina"), String::from("Watkins"), String::from("Barleycorns"));

    let mut alex = Players::new();
    alex.new_player(String::from("Alex"), String::from("Patel"), String::from("32ers"));

    let mut jill = Players::new();
    jill.new_player(String::from("Jill"), String::from("Huang"), String::from("Barleycorns"));

    let mut wanda = Players::new();
    wanda.new_player(String::from("Wanda"), String::from("Vakulskas"), String::from("Barleycorns"));

    let football_players: Vec<Players> = Vec::from([hanzla, tina, alex, jill, wanda]);

    println!("\n---------- Ejercicio 1: ----------");
    println!("Los jugadores que estan en ambos equipos son: {:?}", sports_players(basketball_players, football_players));
}