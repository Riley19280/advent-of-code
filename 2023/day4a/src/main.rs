use std::fs;

struct Card {
    id: i32,
    winning_nums: Vec<i32>,
    my_nums: Vec<i32>,
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let total = contents.split("\n").filter(|x| x != &"").map(|line| {
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
        let count = card.my_nums.iter().fold(0, |acc, num| if card.winning_nums.contains(&num) { acc + 1 } else { acc });

        return if count > 0 {
            i32::pow(2, count - 1)
        } else {
            0
        };
    }).fold(0, |acc, score| acc + score);

    print!("Total: {}", total)
}
