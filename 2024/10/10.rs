use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn parse_file(file_path: &str) -> Vec<Vec<i8>> {
    let mut graph: Vec<Vec<i8>> = Vec::new();
    let path = Path::new(file_path);

    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line").trim().to_string();

        let digits: Vec<i8> = line
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap() as i8)
            .collect();

        graph.push(digits);
    }

    graph
}

fn find_trails(
    graph: &Vec<Vec<i8>>, 
    directions: &Vec<(i32, i32)>, 
    trails: &mut HashSet<(i32, i32, i32, i32)>, 
    visited: &mut HashSet<(i32, i32)>,
    s_row: i32,
    s_col: i32, 
    prev: i8, 
    start_r: i32, 
    start_e:i32
)  {
    if s_col < 0 || s_col >= graph[0].len() as i32 || s_row < 0 || s_row >= graph.len() as i32 {
        return;
    }
    if visited.contains(&(s_row, s_col)) {
        return;
    }
    visited.insert((s_row, s_col));

    if !(s_col == start_e && s_row == start_r) && graph[s_row as usize][s_col as usize] - prev != 1 {
        return;
    }
    if graph[s_row as usize][s_col as usize] == 9 {
        trails.insert((s_row, s_col, start_r, start_e));
        return;
    }
    for dir in directions {
        let mut visited_copy = visited.clone();
        find_trails(graph, directions, trails, &mut visited_copy, s_row+dir.0, s_col+dir.1, graph[s_row as usize][s_col as usize], start_r, start_e);
    }
}

fn main() {
    let path = "/home/minindu/AoC/2024/10/10.txt";
    let graph = parse_file(path);
    let directions: Vec<(i32, i32)> = vec![
        (1, 0),  // Up
        (0, 1),  // Right
        (-1, 0), // Down
        (0, -1), // Left
    ];
    let mut trails: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    for i in 0..graph.len() {
        for j in 0..graph[0].len() {
            if graph[i][j] == 0 {
                let mut visited: HashSet<(i32, i32)> = HashSet::new();
                find_trails(
                        &graph,
                        &directions, 
                        &mut trails, 
                        &mut visited,
                        i as i32, 
                        j as i32, 
                        0, 
                        i as i32, 
                        j as i32
                    );
            }
        }
    }
    println!("Trail {:?}", trails);
    println!("Number of trails: {}", trails.len());
}
