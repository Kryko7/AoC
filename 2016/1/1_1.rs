use std::fs;

fn parsing(filename: &str) -> Vec<(char, i32)> {
    let content = fs::read_to_string(filename)
        .expect("Failed to read file");
    let instructions: Vec<String> = content
        .split(',')
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

fn compute(instructions: Vec<(char, i32)>) -> i64 {
    let mut current = 0; // 0-UP, 1-DOWN, 2-LEFT, 3-RIGHT
    let mut x = 0;
    let mut y = 0;
    for (dir, value) in instructions {
        if current == 0 {
            if dir == 'L' {
                x -= value;
                current = 2;
            } else {
                x += value;
                current = 3;
            }
        } else if current == 1 {
            if dir == 'L' {
                x += value;
                current = 3;
            } else {
                x -= value;
                current = 2;
            }
        } else if current == 2 {
            if dir == 'L' {
                y -= value;
                current = 1;
            } else {
                y += value;
                current = 0;
            }
        } else {
            if dir == 'L' {
                y += value;
                current = 0;
            } else {
                y -= value;
                current = 1;
            }
        }
    }
    (x.abs()+y.abs()).into()
}

fn main() {
    let filename = "/home/minindu/personal/AoC/rust_proj/src/input.txt";
    let instructions = parsing(filename);
    let distance = compute(instructions);
    println!("{:?}", distance);
}
