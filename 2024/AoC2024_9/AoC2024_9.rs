use std::fs::File;
use std::io::{Read};
use std::path::Path;

fn parsing(file_path: &str) -> String {
    let path = Path::new(file_path);
    let mut file = File::open(path).expect("Failed to open file");
    let mut result = String::new();
    
    file.read_to_string(&mut result).expect("Failed to read file");
    
    result
}

fn mapping(data: String) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let mut number = 0;
    let mut index = 0;
    for c in data.chars() {
        let n = c.to_string().parse::<i64>().unwrap();
        if index % 2 == 0 {
            for _ in 0..n {
                result.push(number);
            }
            number += 1;
        } else {
            for _ in 0..n {
                result.push(-1);
            } 
        }
        index += 1;
    }
    result
}

fn swapping(mut data: Vec<i64>) -> Vec<i64> {
    let mut left = 0;
    let mut right = data.len() - 1;
    while left < right {
        while left < right && data[left] != -1 {
            left += 1;
        }
        while left < right && data[right] == -1 {
            right -= 1;
        }
        if left < right {
            data.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
    data
}

fn calculate_checksum(data: Vec<i64>) -> i64 {
    let mut checksum: i64 = 0;
    let mut index = 0;
    while index < data.len() && data[index] != -1 {
        checksum += data[index] * index as i64;
        index += 1;
    }

    checksum
}

fn main() {
    let data = parsing("AoC2024_9.txt");
    let mapped_data = mapping(data);
    let swapped_data = swapping(mapped_data);
    let checksum = calculate_checksum(swapped_data);
    println!("Checksum: {}", checksum);
}