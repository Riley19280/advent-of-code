use std::{cmp, fs};
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,

}


fn get_box_points(schematic: Vec<Vec<char>>, start: Point, end: Point) -> (Point, Point) {
    let tl = Point {
        x: cmp::max(start.x - 1, 0),
        y: cmp::max(start.y - 1, 0),
    };

    let br = Point {
        x: cmp::min(end.x + 1, (schematic[0].len() - 1) as i32),
        y: cmp::min(end.y + 1, (schematic.len() - 1) as i32),
    };


    (tl, br)
}

fn chars_in_box(schematic: Vec<Vec<char>>, start: Point, end: Point) -> Vec<char> {
    let pts = get_box_points(schematic.clone(), start.clone(), end.clone());
    let tl = pts.0;
    let br = pts.1;

    let mut chars: Vec<char> = Vec::new();

    for y in tl.y..br.y + 1 {
        for x in tl.x..br.x + 1 {
            chars.push(schematic[y as usize][x as usize])
        }
    }

    chars
}


fn stars_in_box(schematic: Vec<Vec<char>>, start: Point, end: Point) -> Vec<Point> {
    let pts = get_box_points(schematic.clone(), start.clone(), end.clone());
    let tl = pts.0;
    let br = pts.1;

    let mut points: Vec<Point> = Vec::new();

    for y in tl.y..br.y + 1 {
        for x in tl.x..br.x + 1 {
            if schematic[y as usize][x as usize] == '*' {
                points.push(Point { x, y })
            }
        }
    }

    points
}

fn do_total(gears: &mut HashMap<Point, Vec<i32>>, schematic: Vec<Vec<char>>, start: Option<Point>, last: Option<Point>) {
    if start.is_some() && last.is_some() {
        let chars = chars_in_box(schematic.clone(), start.clone().unwrap(), last.clone().unwrap());

        let stars = stars_in_box(schematic.clone(), start.clone().unwrap(), last.clone().unwrap());

        let s: String = chars.into_iter().collect();

        let nums = s.chars().filter(|c| c.is_numeric()).collect::<String>();

        let num = nums.parse::<i32>().unwrap();


        for star in stars {
            gears.entry(star).and_modify(|v| v.push(num)).or_insert(Vec::from([num]));
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let schematic: Vec<Vec<char>> = contents.split("\n").filter(|x| x != &"").map(|x| x.chars().collect()).collect();

    let mut gears: HashMap<Point, Vec<i32>> = HashMap::new();

    let mut total = 0;

    for y in 0..schematic.len() {
        let mut start: Option<Point> = None;
        let mut last: Option<Point> = None;
        for x in 0..schematic[0].len() {
            let cur_char = schematic[y][x];

            if cur_char.is_numeric() {
                if start.is_none() {
                    start = Option::from(Point { x: (x as i32), y: (y as i32) });
                    last = Option::from(Point { x: (x as i32), y: (y as i32) });
                } else {
                    last = Option::from(Point { x: (x as i32), y: (y as i32) });
                }
            } else {
                do_total(&mut gears, schematic.clone(), start.clone(), last.clone());


                start = None;
                last = None;
            }
        }

        // Handle scenarios where numbers are at the end of the line
        do_total(&mut gears, schematic.clone(), start.clone(), last.clone());
    }


    for (_, values) in gears.iter() {
        if values.len() == 2 {
            total += values[0] * values[1]
        }
    }

    println!("Total: {}", total);
}
