use std::fs;

struct Output {
    fresh_ranges: Vec<(u64, u64)>,
    ingredient_ids: Vec<u64>,
}

fn parsing(filename: &str) -> Output {
    let content = fs::read_to_string(filename)
        .expect("Failed to read file");
    
    let input: Vec<String> = content
        .lines()
        .map(|s| s.trim().to_string())
        .collect();

    let mut mid = false;
    let mut fresh_ranges:Vec<(u64, u64)> = Vec::new();
    let mut ingredient_ids: Vec<u64> = Vec::new();
    for line in input {
        if line == "" {
            mid = true;
            continue;
        }
        if mid {
            let id: u64 = line.parse().unwrap();
            ingredient_ids.push(id);
        } else {
            let (start, end) = match line.split_once('-') {
                Some((a, b)) => (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()),
                None => panic!("Invalid range: no '-' found"),
            };
            fresh_ranges.push((start, end));
        }
    }
    Output { fresh_ranges, ingredient_ids }
}


fn merge_ranges(fresh_ranges: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    fresh_ranges.sort_by(|a, b| b.0.cmp(&a.0));
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    while fresh_ranges.len() > 0 {
        let mut range = fresh_ranges.pop().unwrap();
        if fresh_ranges.len() == 0 {
            merged_ranges.push(range);
        } else {
            if range.1 >= fresh_ranges.last().unwrap().0 {
                let another = fresh_ranges.pop().unwrap();
                range.1 = range.1.max(another.1);
                fresh_ranges.push(range);
            } else {
                merged_ranges.push(range);
            }
        }
    }
    merged_ranges
}

fn fresh_ingredients(fresh_ranges:  &Vec<(u64, u64)>, ingredients: &Vec<u64>) -> u32 {
    let mut count:u32 = 0;
    for ingredient in ingredients {
        for range in fresh_ranges {
            if range.0 <= *ingredient && range.1 >= *ingredient {
                count += 1;
                break;
            }
        }
    }
    count
}

fn all_fresh_ingredients(fresh_ranges:  &Vec<(u64, u64)>) -> u64 {
    let mut count = 0;
    for range in fresh_ranges {
        count += range.1 - range.0 + 1;
    }
    return count;
}


fn main() {
    let filename = "/home/minindu/personal/AoC/rust_proj/src/input.txt";
    let instructions = parsing(filename);
    let mut fresh_ranges = instructions.fresh_ranges;
    let ingredients = instructions.ingredient_ids;
    let merged_ranges = merge_ranges(&mut fresh_ranges);
    let fresh_ingredients = fresh_ingredients(&merged_ranges, &ingredients);
    let total_fresh_ingredients = all_fresh_ingredients(&merged_ranges);
    println!("Fresh Ingredients: {:?}", fresh_ingredients);
    println!("All Fresh Ingredients: {:?}", total_fresh_ingredients);
}
