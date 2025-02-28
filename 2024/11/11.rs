use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::time::Instant;

fn parse_file(file_path: &str) -> HashMap<i64, i64> {
    let mut map = HashMap::new();
    let path = Path::new(file_path);

    let file = File::open(path).expect("Failed to open the file.");
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line").trim().to_string();
        let stones: Vec<i64> = line
                                .trim()
                                .split(" ")
                                .map(|num| num.parse::<i64>().expect("Invalid number"))
                                .collect();
        
        for stone in stones {
            *map.entry(stone).or_insert(0) += 1;
        }
    }
    map
}


fn calc_stones(map: &mut HashMap<i64, i64>, blinks: i64) -> i64 {
    let mut i = 0;
    while i < blinks {
        let mut new_map = HashMap::new();
        for (key, value) in map.iter() {
            if *key == 0 {
                *new_map.entry(1).or_insert(0) += value;
            } else {
                let len = key.to_string().len() as i64;
                if len % 2 == 0 {
                    let power = (len / 2) as u32;
                    *new_map.entry(key / 10_i64.pow(power)).or_insert(0) += value;
                    *new_map.entry(key % 10_i64.pow(power)).or_insert(0) += value;
                } else {
                    *new_map.entry(key * 2024).or_insert(0) += value;
                }
            }
        }
        *map = new_map;
        i += 1;
    }
    let mut total_stones = 0;
    for (_, value) in map.iter() { 
        total_stones += value;
    }
    total_stones
}

fn main() {
    let path = "/home/ishvalin/personal/AoC/2024/11/11.txt";
    
    let parse_start = Instant::now();
    let mut map = parse_file(path);
    let parse_duration = parse_start.elapsed();
    
    let calc_start = Instant::now();
    let total_stones = calc_stones(&mut map, 75);
    let calc_duration = calc_start.elapsed();
    
    println!("Parsing file took: {:?}", parse_duration);
    println!("Calculating stones took: {:?}", calc_duration);
    println!("Total time: {:?}", parse_duration + calc_duration);
    println!("Result: {:?}", total_stones);
}