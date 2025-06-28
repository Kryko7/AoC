use std::{fs::File, io::{BufRead, BufReader}, path::Path};

fn parser(filepath: &str) -> String {
    let path = Path::new(filepath);
    let file = File::open(path).expect("Failed to open the file.");
    let reader = BufReader::new(file);
    let mut output  = String::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        output = line;
    }

    output
}

fn advent_coin_5(input: &str) -> i32 {
    let mut i = 0;
    let mut found = false;
    while !found {
        let s = input.to_string() + &i.to_string();
        let digest = md5::compute(s);
        let md5_hash = format!("{:x}", digest);
        if md5_hash[0..5] == *"00000" {
            found = true;
        }
        i += 1;
    }
    return  i-1;
}

fn advent_coin_6(input: &str) -> i32 {
    let mut i = 0;
    let mut found = false;
    while !found {
        let s = input.to_string() + &i.to_string();
        let digest = md5::compute(s);
        let md5_hash = format!("{:x}", digest);
        if md5_hash[0..6] == *"000000" {
            found = true;
        }
        i += 1;
    }
    return  i-1;
}



fn main() {
    let filepath = "4.txt";
    let input = parser(filepath);
    let ans1 = advent_coin_5(&input);
    let ans2 = advent_coin_6(&input);
    println!("Answer for part1: {}", ans1);
    println!("Answer for part2: {}", ans2);
}