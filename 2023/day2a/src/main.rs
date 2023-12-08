use std::collections::HashMap;
use std::fs;

struct Game<'a> {
    id: i32,
    rounds: Vec<HashMap<&'a str, i32>>,
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let comparison_set = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

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
        let mut valid = true;

        for round in game.rounds {
            for key in comparison_set.keys() {
                if round.get(key).unwrap_or(&0) > comparison_set.get(key).unwrap() {
                    valid = false;
                }
            }
        }

        if valid {
            return acc + game.id;
        }

        acc
    });

    println!("Total: {}", total)
}
