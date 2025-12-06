use std::fs;

fn parsing(filename: &str) -> Vec<(char, i32)> {
    let content = fs::read_to_string(filename)
        .expect("Failed to read file");
    let instructions: Vec<String> = content
        .split('\n')
        .map(|s| s.trim().to_string())
        .collect();
    let parsed: Vec<(char, i32)> = instructions
        .iter()
        .map(|s| {
            let (first, rest) = s.split_at(1);
            let direction = first.chars().next().unwrap();
            let value: i32 = rest.parse().expect("Expected a number after the first character");
            (direction, value)
        })
        .collect();
    parsed
}

fn compute_password1(instructions: Vec<(char, i32)>) -> i32 {
    let mut dial: i32 = 50;
    let mut count: i32 = 0;
    for (dir, dist) in instructions.iter() {
        if *dir == 'L' {
            dial = (dial - dist).rem_euclid(100);
        } else {
            dial = (dial + dist).rem_euclid(100);
        }
        if dial == 0 {
            count += 1;
        }
    }
    count
}

fn compute_password2(instructions: Vec<(char, i32)>) -> i32 {
    let mut dial: i32 = 50;
    let mut count: i32 = 0;
    for (dir, dist) in instructions.iter() {
        if *dir == 'L' {
            for _ in 0..*dist {
                dial = (dial - 1).rem_euclid(100);
                if dial == 0 {
                    count += 1;
                }
            }
            
        } else {
            for _ in 0..*dist {
                dial = (dial + 1).rem_euclid(100);
                if dial == 0 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let filename = "/home/minindu/personal/AoC/rust_proj/src/input.txt";
    let instructions = parsing(filename);
    let password1 = compute_password1(instructions.clone());
    let password2 = compute_password2(instructions);
    println!("Password 1: {}", password1);
    println!("Password 2: {}", password2);
}
