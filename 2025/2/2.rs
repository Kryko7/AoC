use std::fs;
use std::cmp::min;

fn parsing(filename: &str) -> Vec<(u128, u128)> {
    let content = fs::read_to_string(filename)
        .expect("Failed to read file");
    let instructions: Vec<String> = content
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let ranges: Vec<(u128, u128)> = instructions
        .iter()
        .map(|s| {
            let (start, end) = s.split_once('-').expect("Expected start-end format");
            let start_range: u128 = start.parse().expect("Expected a number");
            let end_range: u128 = end.parse().expect("Expected a number");
            (start_range, end_range)
        })
        .collect();
    ranges
}

fn invalid_id_sum1(ranges: Vec<(u128, u128)>) -> u128 {
    let mut id_sum: u128 = 0;
    for (start, end) in ranges.iter() {
        for i in *start..*end+1 {
            let s = i.to_string();
            if s.len() % 2 != 0 {
                continue;
            }
            let (a, b) = s.split_at(s.len() / 2);
            let x: u128 = a.parse().expect("Expected a number");
            let y: u128 = b.parse().expect("Expected a number");
            if x == y {
                id_sum += i;
            }
        }
    }
    id_sum
}

fn invalid_id_sum2(ranges: Vec<(u128, u128)>) -> u128 {
    let mut id_sum: u128 = 0;
    for (start, end) in ranges.iter() {
        for i in *start..*end+1 {
            let s = i.to_string();
            let mut map: [usize; 10] = [0; 10];
            for c in s.chars() {
                if let Some(digit) = c.to_digit(10) {
                    map[digit as usize] += 1;
                }
            }
            let mut splits: usize= 1000;
            
            for j in 0..10 {
                if map[j] != 0 {
                    splits = min(splits, map[j]);
                }
            }
            for split in 2..splits+1 {
                let len = s.len() / split ;
                let mut parts: Vec<i128> = Vec::new();
                let mut start = 0;
                
                for _ in 0..split {
                    let end = start + len;
                    parts.push((&s[start..end]).parse().expect("Expected a number"));
                    start = end;
                }
                if start < s.len() {
                    parts.push((&s[start..]).parse().expect("Expected a number"));
                }
                let num = parts[0];
                let mut invalid = true;
                for x in 1..parts.len() {
                    if num != parts[x] {
                        invalid = false;
                    }
                }
                if parts.len() == 1 {
                    invalid = false;
                }
                if invalid {
                    id_sum += i;
                    break
                }
            }
        }
    }
    id_sum
}

fn main() {
    let filename = "/home/minindu/personal/AoC/rust_proj/src/input.txt";
    let instructions = parsing(filename);
    let id_sum1 = invalid_id_sum1(instructions.clone());
    let id_sum2 = invalid_id_sum2(instructions);
    println!("ID Sum 1 {:?}", id_sum1);
    println!("ID Sum 2 {:?}", id_sum2);

}
