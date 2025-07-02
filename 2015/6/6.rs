use std::{fs::File, io::{BufRead, BufReader}, path::Path};

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

fn lit_lights(input: &Vec<String>) -> i32 {
    let mut arr = vec![vec![false; 1000]; 1000];
    let mut lights = 0;
    for instruct in input {
        if instruct.starts_with("turn on") {
            let parts: Vec<&str> = instruct.split_whitespace().collect();
            let start: Vec<usize> = parts[2]
                                        .split(',')
                                        .map(|s| s.trim())
                                        .filter(|s| !s.is_empty())
                                        .map(|s| s.parse::<usize>().unwrap())
                                        .collect();
            let end: Vec<usize> = parts[4]
                                        .split(',')
                                        .map(|s| s.trim())
                                        .filter(|s| !s.is_empty())
                                        .map(|s| s.parse::<usize>().unwrap())
                                        .collect();
            for i in start[0]..end[0]+1 {
                for j in start[1]..end[1]+1 {
                    arr[i][j] = true;
                }
            }
        } else if instruct.starts_with("turn off") {
            let parts: Vec<&str> = instruct.split_whitespace().collect();
            let start: Vec<usize> = parts[2]
                                        .split(',')
                                        .map(|s| s.trim())
                                        .filter(|s| !s.is_empty())
                                        .map(|s| s.parse::<usize>().unwrap())
                                        .collect();
            let end: Vec<usize> = parts[4]
                                        .split(',')
                                        .map(|s| s.trim())
                                        .filter(|s| !s.is_empty())
                                        .map(|s| s.parse::<usize>().unwrap())
                                        .collect();
            for i in start[0]..end[0]+1 {
                for j in start[1]..end[1]+1 {
                    arr[i][j] = false;
                }
            }
        } else {
            let parts: Vec<&str> = instruct.split_whitespace().collect();
            let start: Vec<usize> = parts[1]
                                        .split(',')
                                        .map(|s| s.trim())
                                        .filter(|s| !s.is_empty())
                                        .map(|s| s.parse::<usize>().unwrap())
                                        .collect();
            let end: Vec<usize> = parts[3]
                                        .split(',')
                                        .map(|s| s.trim())
                                        .filter(|s| !s.is_empty())
                                        .map(|s| s.parse::<usize>().unwrap())
                                        .collect();
            for i in start[0]..end[0]+1 {
                for j in start[1]..end[1]+1 {
                    arr[i][j] = !arr[i][j];
                }
            }
        }
    }
    for i in 0..1000 {
        for j in 0..1000  {
            if arr[i as usize][j as usize] {
                lights += 1;
            }
        }
    }
    lights
}


fn total_brigtness(input: &Vec<String>) -> i32 {
    let mut arr = vec![vec![0; 1000]; 1000];
    let mut brightness = 0;
    for instruct in input {
        if instruct.starts_with("turn on") {
            let parts: Vec<&str> = instruct.split_whitespace().collect();
            let start: Vec<usize> = parts[2]
                                        .split(',')
                                        .map(|s| s.trim())
                                        .filter(|s| !s.is_empty())
                                        .map(|s| s.parse::<usize>().unwrap())
                                        .collect();
            let end: Vec<usize> = parts[4]
                                        .split(',')
                                        .map(|s| s.trim())
                                        .filter(|s| !s.is_empty())
                                        .map(|s| s.parse::<usize>().unwrap())
                                        .collect();
            for i in start[0]..end[0]+1 {
                for j in start[1]..end[1]+1 {
                    arr[i][j] += 1;
                }
            }
        } else if instruct.starts_with("turn off") {
            let parts: Vec<&str> = instruct.split_whitespace().collect();
            let start: Vec<usize> = parts[2]
                                        .split(',')
                                        .map(|s| s.trim())
                                        .filter(|s| !s.is_empty())
                                        .map(|s| s.parse::<usize>().unwrap())
                                        .collect();
            let end: Vec<usize> = parts[4]
                                        .split(',')
                                        .map(|s| s.trim())
                                        .filter(|s| !s.is_empty())
                                        .map(|s| s.parse::<usize>().unwrap())
                                        .collect();
            for i in start[0]..end[0]+1 {
                for j in start[1]..end[1]+1 {
                    if arr[i][j] > 0 {
                        arr[i][j] -= 1;
                    }
                }
            }
        } else {
            let parts: Vec<&str> = instruct.split_whitespace().collect();
            let start: Vec<usize> = parts[1]
                                        .split(',')
                                        .map(|s| s.trim())
                                        .filter(|s| !s.is_empty())
                                        .map(|s| s.parse::<usize>().unwrap())
                                        .collect();
            let end: Vec<usize> = parts[3]
                                        .split(',')
                                        .map(|s| s.trim())
                                        .filter(|s| !s.is_empty())
                                        .map(|s| s.parse::<usize>().unwrap())
                                        .collect();
            for i in start[0]..end[0]+1 {
                for j in start[1]..end[1]+1 {
                    arr[i][j] += 2;
                }
            }
        }
    }
    for i in 0..1000 {
        for j in 0..1000  {
            brightness += arr[i as usize][j as usize];
        }
    }
    brightness
}

fn main() {
    let filepath = "6.txt";
    let input = parser(filepath);
    let ans1 = lit_lights(&input);
    let ans2 = total_brigtness(&input);
    println!("Answer for part1: {}", ans1);
    println!("Answer for part2: {}", ans2);
}