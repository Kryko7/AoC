use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};

fn parse_file(file_path: &str) -> HashMap<i64, i64> {
    let mut map = HashMap::new();
    let path = Path::new(file_path);

    let file = File::open(path).expect("Failed to open the file.");
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line").trim().to_string();
        let lanternfishes: Vec<i64> = line
                                        .trim()
                                        .split(",")
                                        .map(|num| num.parse::<i64>().expect("Invalid number"))
                                        .collect();
        
        for lanternfish in lanternfishes {
            *map.entry(lanternfish).or_insert(0) += 1;
        }
    }
    map
}

fn calc_lanternfishes(map: &mut HashMap<i64, i64>, days: i64) -> i64 {
    let mut i = 0;
    while i < days {
        let mut new_map = HashMap::new();
        for (key, value) in map.iter() {
            if *key == 0 {
                *new_map.entry(6).or_insert(0) += value;
                *new_map.entry(8).or_insert(0) += value;
            } else {
                *new_map.entry(key - 1).or_insert(0) += value;
            }
        }
        *map = new_map;
        i += 1;
    }
    let mut total_fishes = 0;
    for (_, value) in map.iter() { 
        total_fishes += value;
    }
    total_fishes
}

fn main() {
    let path = "/home/ishvalin/personal/AoC/2021/6/6.txt";
    let mut map = parse_file(path);
    let total_fishes = calc_lanternfishes(&mut map, 256);
    println!("{:?}", total_fishes);
}