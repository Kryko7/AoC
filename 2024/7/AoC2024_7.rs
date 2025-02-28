use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Equation {
    index: usize,
    target: i64,
    equation: Vec<i64>,
}

fn parse_file(file_path: &str) -> Vec<Equation> {
    let mut equations: Vec<Equation> = Vec::new();
    let path = Path::new(file_path);

    // Open the file and create a buffered reader
    let file = File::open(path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line").trim().to_string();

        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        // Split the line into target and equation parts
        let parts: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
        if parts.len() != 2 {
            eprintln!("Malformed line (index {}): {}", index, line);
            continue;
        }

        // Parse the target
        let target = match parts[0].parse::<i64>() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Failed to parse target (index {}): {}", index, line);
                continue;
            }
        };

        // Parse the equation
        let equation: Result<Vec<i64>, _> = parts[1]
            .split_whitespace()
            .map(|x| x.parse::<i64>())
            .collect();
        let equation = match equation {
            Ok(values) => values,
            Err(_) => {
                eprintln!("Failed to parse equation (index {}): {}", index, line);
                continue;
            }
        };

        // Add the parsed equation to the list
        equations.push(Equation {
            index: index,
            target,
            equation,
        });
    }

    equations
}

fn solve_equation(equation: &[i64], target: i64, index: usize, curr_sum: i64) -> bool {
    if index == equation.len() {
        return target == curr_sum;
    }
    if solve_equation(equation, target, index + 1, curr_sum + equation[index]) {
        return true;
    }
    if solve_equation(equation, target, index + 1, curr_sum * equation[index]) {
        return true;
    }
    false

}

fn solve_equation2(equation: &[i64], target: i64, index: usize, curr_sum: i64) -> bool {
    if index == equation.len() {
        return target == curr_sum;
    }
    if solve_equation2(equation, target, index + 1, curr_sum + equation[index]) {
        return true;
    }
    if solve_equation2(equation, target, index + 1, curr_sum * equation[index]) {
        return true;
    }
    let a = curr_sum.to_string();
    let b = equation[index].to_string();

    let concatanated = format!("{}{}", a, b).parse::<i64>().unwrap();
    if solve_equation2(equation, target, index + 1, concatanated) {
        return true;
    }
    false
}

fn main() {
    let equations = parse_file("AoC2024_7.txt");
    // println!("{:?}", equations);
    let mut total: i64 = 0;
    let mut total2: i64 = 0;
    for equation in equations {
        if solve_equation(&equation.equation, equation.target, 1, equation.equation[0]) {
            total += equation.target;
        }
        if solve_equation2(&equation.equation, equation.target, 1, equation.equation[0]) {
            total2 += equation.target;
        }
    }
    println!("Total: {}", total);
    println!("Total2: {}", total2);
}