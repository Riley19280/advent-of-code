use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut total = 0;

    for line in lines {
        if line == "" {
            continue;
        }

        let mut new_string = String::new();

        for char_idx in 0..line.len() {
            if line.chars().nth(char_idx).unwrap().is_numeric() {
                new_string = format!("{}{}", new_string, line.chars().nth(char_idx).unwrap())
            }
        }

        let first_num = new_string.chars().nth(0).unwrap();
        let last_num = new_string.chars().nth(new_string.len() - 1).unwrap();

        let number = format!("{}{}", first_num, last_num).parse::<i32>().unwrap();

        total += number
    }

    println!("Total: {}", total)
}
