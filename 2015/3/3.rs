use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}, path::Path};

fn parser(filepath: &str) -> Vec<char> {
    let path = Path::new(filepath);
    let file = File::open(path).expect("Failed to open the file.");
    let reader = BufReader::new(file);
    let mut output: Vec<char> = vec![];

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts = line.chars().collect();
        output = parts;
    }

    output
}

fn unique_houses(input: &Vec<char>) -> usize {
    let mut set = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    set.insert((x, y));
    for c in input {
        if *c == '>' {
            x += 1;
        } else if *c == '<' {
            x -= 1;
        } else if *c == '^' {
            y += 1;
        } else {
            y -= 1;
        }
        set.insert((x, y));
    }
    set.len()
}

fn unique_houses2(input: &Vec<char>) -> usize {
    let mut set = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    let mut a = 0;
    let mut b = 0;
    set.insert((x, y));
    for (i, c) in input.iter().enumerate() {
        if i % 2 == 0 {
            if *c == '>' {
                x += 1;
            } else if *c == '<' {
                x -= 1;
            } else if *c == '^' {
                y += 1;
            } else {
                y -= 1;
            }
            set.insert((x, y));
        } else {
            if *c == '>' {
                a -= 1;
            } else if *c == '<'{
                a += 1;
            } else if *c == '^' {
                b -= 1;
            } else {
                b += 1;
            }
            set.insert((a, b));
        }
    }
    set.len()
}


fn main() {
    let filepath = "3.txt";
    let input = parser(filepath);
    let ans1 = unique_houses(&input);
    let ans2 = unique_houses2(&input);
    println!("Answer for part1: {}", ans1);
    println!("Answer for part2: {}", ans2);
}