use std::fs;
use std::cmp::max;

fn parsing(filename: &str) -> Vec<Vec<u32>> {
    let content = fs::read_to_string(filename)
        .expect("Failed to read file");
    
    let battery_banks = content
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect()
        })
        .collect();
    
    battery_banks
}

fn compute_joltage_1(battery_banks: &Vec<Vec<u32>>) -> u64 {
    let mut total_joltage = 0;
    
    for battery_bank in battery_banks {
        let n = battery_bank.len();
        let mut pre_max = vec![0; n];
        let mut post_max = vec![0; n];
        
        pre_max[0] = battery_bank[0];
        post_max[n-1] = battery_bank[n-1];
        
        for i in 1..n {
            pre_max[i] = max(pre_max[i-1], battery_bank[i]);
            post_max[n-i-1] = max(post_max[n-i], battery_bank[n-i-1]);
        }
        
        let mut joltage = 0;
        for i in 0..n-1 {
            joltage = max(joltage, 10 * pre_max[i] as u64 + post_max[i+1] as u64);
        }
        
        total_joltage += joltage
    }
    
    total_joltage
}

fn compute_joltage_2(battery_banks: &Vec<Vec<u32>>) -> u64 {
    let mut total_joltage: u64 = 0;
    
    for battery_bank in battery_banks {
        let n: usize = battery_bank.len();
        let mut dist: usize = n - 11;
        let mut pos: usize = 0;
        let mut joltage: u64 = 0;
        
        for i in (1..=12).rev() {
            let mut digit: u32 = 0;
            let end = dist;
            let start = pos;
            
            for j in start..end {
                if battery_bank[j] > digit {
                    pos = j;
                    digit = battery_bank[j]
                }
            }
            
            joltage += digit as u64 * 10u64.pow((i as u32) - 1);
            pos += 1;
            dist += 1
        }
        
        total_joltage += joltage;
    }
    
    total_joltage
}



fn main() {
    let filename = "/home/minindu/personal/AoC/rust_proj/src/input.txt";
    let instructions = parsing(filename);
    let joltage_1 = compute_joltage_1(&instructions);
    let joltage_2 = compute_joltage_2(&instructions);
    println!("Joltage 1: {:?}", joltage_1);
    println!("Joltage 2: {:?}", joltage_2)

}
