use std::fs;
use std::collections::HashSet;


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
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut set = HashSet::new();
    for (dir, value) in instructions {
        if current == 0 {
            if dir == 'L' {
                for _ in 0..value {
                    x -= 1;
                    if set.contains(&(x, y)) {
                        return (x.abs()+y.abs()).into();
                    }
                    set.insert((x, y));
                }
                current = 2;
            } else {
                for _ in 0..value {
                    x += 1;
                    if set.contains(&(x, y)) {
                        return (x.abs()+y.abs()).into();
                    }
                    set.insert((x, y));
                }
                current = 3;
            }
        } else if current == 1 {
            if dir == 'L' {
                for _ in 0..value {
                    x += 1;
                    if set.contains(&(x, y)) {
                        return (x.abs()+y.abs()).into();
                    }
                    set.insert((x, y));
                }
                current = 3;
            } else {
                for _ in 0..value {
                    x -= 1;
                    if set.contains(&(x, y)) {
                        return (x.abs()+y.abs()).into();
                    }
                    set.insert((x, y));
                }
                current = 2;
            }
        } else if current == 2 {
            if dir == 'L' {
                for _ in 0..value {
                    y -= 1;
                    if set.contains(&(x, y)) {
                        return (x.abs()+y.abs()).into();
                    }
                    set.insert((x, y));
                }
                current = 1;
            } else {
                for _ in 0..value {
                    y += 1;
                    if set.contains(&(x, y)) {
                        return (x.abs()+y.abs()).into();
                    }
                    set.insert((x, y));
                }
                current = 0;
            }
        } else {
            if dir == 'L' {
                for _ in 0..value {
                    y += 1;
                    if set.contains(&(x, y)) {
                        return (x.abs()+y.abs()).into();
                    }
                    set.insert((x, y));
                }
                current = 0;
            } else {
                for _ in 0..value {
                    y -= 1;
                    if set.contains(&(x, y)) {
                        return (x.abs()+y.abs()).into();
                    }
                    set.insert((x, y));
                }
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
