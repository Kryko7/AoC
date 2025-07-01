use std::{fs::File, io::{BufRead, BufReader}, path::Path};
use fancy_regex::Regex;

fn parser(filepath: &str) -> Vec<String> {
    let path = Path::new(filepath);
    let file = File::open(path).expect("Failed to open the file.");
    let reader = BufReader::new(file);
    let mut output  = vec![];

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        output.push(line);
    }

    output
}

fn has_twice_in_row(text: &str) -> bool {
    let mut prev = None;
    for c in text.chars() {
        if Some(c) == prev {
            return true;
        }
        prev = Some(c);
    }
    false
}

fn has_twice_the_pattern(text: &str) -> bool {
    let re = Regex::new(r"(..).*\1").unwrap();
    re.is_match(text).unwrap()
}

fn contains_forbidden_pattern(text: &str) -> bool {
    let re = Regex::new(r"ab|cd|pq|xy").unwrap();
    re.is_match(text).unwrap()
}

fn count_vowels(text: &str) -> bool {
    let re = Regex::new(r"[aeiou]").unwrap();
    re.find_iter(text).count() >= 3
}
fn matches_pattern(text: &str) -> bool {
    let re = Regex::new(r"([a-zA-Z]).\1").unwrap();
    re.is_match(text).unwrap()
}

fn is_nice(text: &str) -> bool {
    has_twice_in_row(text) && !contains_forbidden_pattern(text) && count_vowels(text)
}

fn is_nice2(text: &str) -> bool {
    has_twice_the_pattern(text) && matches_pattern(text)
}

fn nice_count(input: &Vec<String>) -> i32 {
    let mut result = 0;
    for text in input {
        if is_nice(text) {
            result += 1;
        }
    }
    result
}

fn nice_count2(input: &Vec<String>) -> i32 {
    let mut result = 0;
    for text in input {
        if is_nice2(text) {
            result += 1
        }
    }
    result
}

fn main() {
    let filepath = "5.txt";
    let input = parser(filepath);
    let ans1 = nice_count(&input);
    let ans2 = nice_count2(&input);
    println!("Answer for part1: {}", ans1);
    println!("Answer for part2: {}", ans2);
}