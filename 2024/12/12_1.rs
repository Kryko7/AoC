use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};

fn parse_file(file_path: &str) -> Vec<Vec<char>> {
    let mut graph: Vec<Vec<char>> = Vec::new();
    let path = Path::new(file_path);

    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line").trim().to_string();

        let chars: Vec<char> = line.chars().collect();

        graph.push(chars);
    }
    println!("{:?}", graph);
    graph
}

fn total_price(graph: &Vec<Vec<char>>, directions: &Vec<(i32, i32)>, visited: &mut Vec<Vec<bool>>, area: &mut i32, perimiter: &mut i32, i: i32, j: i32, region: char) {
    if i < 0 || j < 0 || i >= graph.len() as i32 || j >= graph[0].len() as i32 || visited[i as usize][j as usize] || graph[i as usize][j as usize] != region{
        return;
    }
    visited[i as usize][j as usize] = true;
    *area += 1;
    if i + 1 < graph.len() as i32 {
        if graph[(i+1) as usize][j as usize] != graph[i as usize][j as usize] {
            *perimiter += 1;
        }
    } else {
        *perimiter += 1;
    }
    if i - 1 >= 0 {
        if graph[(i-1) as usize][j as usize] != graph[i as usize][j as usize] {
            *perimiter += 1;
        }
    } else {
        *perimiter += 1;
    }
    if j + 1 < graph[0].len() as i32 {
        if graph[i as usize][(j+1) as usize] != graph[i as usize][j as usize] {
            *perimiter += 1;
        }
    } else {
        *perimiter += 1;
    }
    if j - 1 >= 0 {
        if graph[i as usize][(j-1) as usize] != graph[i as usize][j as usize] {
            *perimiter += 1;
        }
    } else {
        *perimiter += 1;
    }
    for dir in directions {
        total_price(graph, directions, visited, area, perimiter, i + dir.0, j + dir.1, region)
    }
}

fn main() {
    let path = "/home/minindu/AoC/2024/12/12.txt";
    let graph = parse_file(path);
    let directions: Vec<(i32, i32)> = vec![
        (1, 0),  // Up
        (0, 1),  // Right
        (-1, 0), // Down
        (0, -1), // Left
    ];
    let rows = graph.len();
    let columns = graph[0].len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; columns]; rows];
    let mut total = 0;
    for i in 0..graph.len() {
        for j in 0..graph[0].len() {
            let mut area = 0;
            let mut perimiter = 0;
            if !visited[i][j] {
                total_price(&graph, &directions, &mut visited, &mut area, &mut perimiter, i as i32, j as i32, graph[i][j]);
                println!("{} {} {}", graph[i][j], area, perimiter);
                total += area * perimiter;
            }
        }
    }
    println!("Total: {}", total);
}