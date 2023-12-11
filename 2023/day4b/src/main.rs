use std::collections::HashMap;
use std::fs;

struct Card {
    id: i32,
    winning_nums: Vec<i32>,
    my_nums: Vec<i32>,
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut instances: HashMap<i32, i32> = HashMap::new();

    let _ = contents.split("\n").filter(|x| x != &"").map(|line| {
        let str1 = line.split(":").collect::<Vec<&str>>();
        let id = str1[0].trim().replace("Card", "").trim().to_owned().parse::<i32>().unwrap();

        let str2 = str1[1].split("|").collect::<Vec<&str>>();

        let winning_nums = str2[0].split(" ").map(|x| x.trim()).filter(|x| x != &"").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let my_nums = str2[1].split(" ").map(|x| x.trim()).filter(|x| x != &"").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        Card {
            id,
            winning_nums,
            my_nums,
        }
    }).map(|card| {
        let winning_count = card.my_nums.iter().fold(0, |acc, num| if card.winning_nums.contains(&num) { acc + 1 } else { acc });

        instances.entry(card.id).and_modify(|x| *x += 1).or_insert(1);

        let instance_count = *instances.get(&card.id).unwrap();

        println!("Card {} has {} instances and {} winning numbers", card.id, instance_count, winning_count);

        for next_id in card.id + 1..card.id + winning_count + 1 {
            instances.entry(next_id).and_modify(|x| *x += instance_count).or_insert(instance_count);
        }
    }).collect::<()>();

    let total = instances.iter().fold(0, |acc, (id, total)| acc + total);

    println!("Total Cards: {}", total)
}
