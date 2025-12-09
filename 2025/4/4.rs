use std::fs;

fn parsing(filename: &str) -> Vec<Vec<i32>> {
    let content = fs::read_to_string(filename)
        .expect("Failed to read file");
    
    let grid = content
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| {
                    if c == '.' {
                        return Some(0);
                    } else {
                        return Some(1);
                    }
                })
                .collect()
        })
        .collect();
    
    grid
}

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (1, 1),
    (-1, 0),
    (0, -1),
    (-1, -1),
    (-1, 1),
    (1, -1),
];

fn liftable_roll_papers(grid: &Vec<Vec<i32>>)  -> i32 {
     let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    let mut liftable = 0;

    for x in 0..n {
        for y in 0..m {
            if grid[x as usize][y as usize] == 1 {
                let mut rolls = 0;
                for direction in DIRECTIONS {
                    let x_new = x + direction.0;
                    let y_new = y + direction.1;
                    if x_new >= 0 && x_new < n && y_new >= 0 && y_new < m {
                        if grid[x_new as usize][y_new as usize] == 1 {
                            rolls += 1;
                        }
                    }
                }
                if rolls < 4 {
                    liftable += 1;
                }
            }
        }
    }
    liftable
}

fn removable_roll_papers(grid: &mut Vec<Vec<i32>>)  -> i32 {
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    let mut removed = -1;
    let mut total_removed = 0;

    while removed != 0 {
        removed = 0;
        for x in 0..n {
            for y in 0..m {
                if grid[x as usize][y as usize] == 1 {
                    let mut rolls = 0;
                    for direction in DIRECTIONS {
                        let x_new = x + direction.0;
                        let y_new = y + direction.1;
                        if x_new >= 0 && x_new < n && y_new >= 0 && y_new < m {
                            if grid[x_new as usize][y_new as usize] == 1 {
                                rolls += 1;
                            }
                        }
                    }
                    if rolls < 4 {
                        grid[x as usize][y as usize] = 0;
                        removed += 1;
                    }
                }
            }
        }
        total_removed += removed;
    }

    total_removed
}




fn main() {
    let filename = "/home/minindu/personal/AoC/rust_proj/src/input.txt";
    let mut instructions = parsing(filename);
    let liftable_rolls = liftable_roll_papers(&instructions);
    let removable_rolls = removable_roll_papers(&mut instructions);
    println!("Liftable rolls: {:?}", liftable_rolls);
    println!("Removable rolls: {:?}", removable_rolls);
}
