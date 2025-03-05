use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path};
use std::cmp::max;
use regex::Regex;

#[derive(Debug)]
struct Game {
    number: u32,
    data: Vec<HashMap<String, u32>>,
}


fn parser(filepath: &str) -> Vec<Game> {
    let path = Path::new(filepath);
    let file = File::open(path).expect("Failed to open the file");
    let reader = BufReader::new(file);

    let mut parsed: Vec<Game> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let input = line.expect("Failed to read line");
        let re = Regex::new(r"Game (\d+):").unwrap();
    
        // Extract the game number from the "Game X:" part
        let caps = re.captures(&input).expect("Failed to extract game number");
        let game_number: u32 = caps[1].parse().unwrap();

        // Split by the colon (:) to separate the game number from the actual data
        let parts: Vec<&str> = input.splitn(2, ':').collect();

        if parts.len() < 2 {
            panic!("Invalid input format!");
        }

        let game_data = parts[1].trim();

        let mut result = Vec::new();

        let groups: Vec<&str> = game_data.split(';').collect();

        for group in groups {
            let mut colors_count = HashMap::new();

            let pairs: Vec<&str> = group.split(',').collect();

            for pair in pairs {
                let mut parts = pair.trim().split_whitespace();
                if let (Some(count), Some(color)) = (parts.next(), parts.next()) {
                    let count = count.parse::<u32>().unwrap();
                    colors_count.insert(color.to_string(), count);
                }
            }
            result.push(colors_count);
        }

        parsed.push(Game {
            number: game_number,
            data: result,
        });
    }
    parsed
}

fn game_id_sum(games: Vec<Game>) -> u32 {
    let mut count: u32 = 0; 
    for game in games {
        let id = game.number;
        let mut can = true;
        for group in game.data {
            if group.get("red").unwrap_or(&0) <= &12 
                && group.get("green").unwrap_or(&0) <= &13 
                && group.get("blue").unwrap_or(&0) <= &14 {
                continue;
            } else {
                can = false;
            }
        }
        if can {
            count += id;
        }
    }
    count
}


fn main() {
    let filepath = "src/text.txt";
    let result:Vec<Game> = parser(filepath);
    let game_count = game_id_sum(result);
    println!("Result is: {}", game_count);
}