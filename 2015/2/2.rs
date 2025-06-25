use std::{fs::File, io::{BufRead, BufReader}, path::Path};

fn parser(filepath: &str) -> Vec<Vec<i32>> {
    let path = Path::new(filepath);
    let file = File::open(path).expect("Failed to open the file.");
    let reader = BufReader::new(file);
    let mut output: Vec<Vec<i32>> = vec![];

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let dimensions: Vec<i32> = line
            .split('x')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        output.push(dimensions);
    }

    output
}

fn wrapping_paper(input: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for v in input {
        let wrapping = 2*v[0]*v[1] + 2*v[0]*v[2] + 2*v[1]*v[2];

        let slack = find_min_area(v);
        total += wrapping + slack;
    }
    total
}

fn ribbons(input: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for v in input {
        let wrapping = v[0] * v[1] * v[2];

        let slack = find_min_perimeter(v);
        total += wrapping + slack;
    }
    total    
}

fn find_min_area(sides: &Vec<i32>) -> i32 {
    let mut sorted = sides.clone();
    sorted.sort();

    sorted[0] * sorted[1]
}

fn find_min_perimeter(sides: &Vec<i32>) -> i32 {
    let mut sorted: Vec<i32> = sides.clone();
    sorted.sort();

    sorted[0] * 2 + sorted[1] * 2
}


fn main() {
    let filepath = "2.txt";
    let input = parser(filepath);
    let ans1 = wrapping_paper(&input);
    let ans2 = ribbons(&input);
    println!("Answer for part1: {}", ans1);
    println!("Answer for part2: {}", ans2);
}