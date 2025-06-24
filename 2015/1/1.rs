use std::{fs::File, io::{BufReader, Read}, path::Path};

fn parser(filepath: &str) -> String {
    let path = Path::new(filepath);
    let file = File::open(path).expect("Failed to open the file.");
    let mut reader = BufReader::new(file);
    let mut text = String::new();
    reader.read_to_string(&mut text).expect("Failed to read the file.");
    text
}

fn floor_counter(text: &String) -> i32 {
    let mut floor = 0;
    for c in text.chars() {
        if c == '(' {
            floor += 1;
        } else {
            floor -=1;
        }
    }
    floor
}

fn first_chance(text: &String) -> i32 {
    let mut floor: i32 = 0;
    for (i, c) in text.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
            if floor == -1 {
                return  (i + 1).try_into().unwrap();
            }
        }
    }
    floor
}

fn main() {
    let filepath = "1.txt";
    let text = parser(filepath);
    let ans1 = floor_counter(&text);
    let ans2 = first_chance(&text);
    println!("Answer for part1: {}", ans1);
    println!("Answer for part2: {}", ans2);
}