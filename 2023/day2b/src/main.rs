use std::cmp::min;
use std::collections::HashMap;
use std::fs;

struct Game<'a> {
    id: i32,
    rounds: Vec<HashMap<&'a str, i32>>,
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let games = contents.split("\n").filter(|l| l != &"").map(|line| {
        let game_id = line.split(":").next().unwrap().replace("Game ", "").parse::<i32>().unwrap();

        let rounds = line.split(":").nth(1).unwrap().split(";").map(|str| {
            let colors = str.split(",").map(|s| s.trim()).fold(HashMap::new(), |mut acc, str| {
                acc.insert(
                    str.split(" ").nth(1).unwrap(),
                    str.split(" ").nth(0).unwrap().parse::<i32>().unwrap(),
                );

                acc
            });

            colors
        }).collect();

        Game {
            id: game_id,
            rounds,
        }
    });

    let total = games.fold(0, |acc, game| {
        let mut min_map = HashMap::new();

        for round in game.rounds {
            for (key, val) in round.clone().into_iter() {
                if round.get(key).unwrap() > min_map.get(key).unwrap_or(&0) {
                    min_map.insert(key, *round.get(key).unwrap());
                }
            }
        }

        acc + min_map.into_values().fold(1, |a,x| a * x)
    });

    println!("Total: {}", total)
}
