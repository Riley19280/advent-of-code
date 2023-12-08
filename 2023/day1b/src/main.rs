use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let number_replacements = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut total = 0;

    for line in lines {
        if line == "" {
            continue;
        }

        let mut char_idx = 0;

        let mut new_string = String::new();
        while char_idx < line.len() {
            let partial_line = &line[char_idx..line.len()];

            if partial_line.chars().next().unwrap().is_numeric() {
                new_string = format!("{}{}", new_string, partial_line.chars().next().unwrap())
            } else {
                for repl_idx in 0..number_replacements.len() {
                    let number_replacement = number_replacements[repl_idx];

                    if partial_line.starts_with(number_replacement) {
                        new_string = format!("{}{}", new_string, repl_idx + 1);
                        char_idx += number_replacement.len() - 2;
                    }
                }
            }

            char_idx += 1;
        }

        let first_num = new_string.chars().nth(0).unwrap();
        let last_num = new_string.chars().nth(new_string.len() - 1).unwrap();

        let number = format!("{}{}", first_num, last_num).parse::<i32>().unwrap();

        total += number
    }

    println!("Total: {}", total)
}
