use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}, path::Path};

#[derive(Debug)]
struct Card {
    winning_cards: HashSet<u32>,
    owned_cards: Vec<u32>,
}


fn parsing(file_path: &str) -> Vec<Card> {
    let path = Path::new(file_path);
    let file = File::open(path).expect("Failed to open the file");

    let reader = BufReader::new(file);
    let mut cards: Vec<Card> = Vec::new();

    for (_, line) in reader.lines().enumerate() {
        let card = line.expect("Failed to read the line");
        let parts: Vec<&str> = card.trim().split(":").collect();
        let two_arrays: Vec<&str> = parts[1].trim().split("|").collect();
        let winning_cards: HashSet<u32> = two_arrays[0]
                        .trim()
                        .split_whitespace()
                        .map(|num| num.parse::<u32>().expect("Invalid number"))
                        .collect();
        let owned_cards: Vec<u32> = two_arrays[1]
                        .trim()
                        .split_whitespace()
                        .map(|num| num.parse::<u32>().expect("Invalid number"))
                        .collect();
        cards.push(Card{winning_cards, owned_cards:owned_cards});
    }
    cards
}


fn count(cards: Vec<Card>) -> u32 {
    let mut total:u32 = 0;
    let num_of_cards = &cards.len();
    let mut vec = vec![1; *num_of_cards];
    for i in 0..*num_of_cards {
        let mut count: u32 = 0;
        for num in &cards[i].owned_cards {
            if cards[i].winning_cards.contains(&num) {
                count += 1;
            }
        }
        for j in i+1..(i+1+count as usize) {
            if j > *num_of_cards {
                break;
            }
            vec[j] += vec[i];
        }
    }
    for i in 0..*num_of_cards {
        total += vec[i];
    }
    total
}


fn main() {
    let file_path = "/home/ishvalin/personal/AoC/2023/4/4.txt";
    let cards:Vec<Card> = parsing(file_path);
    let total = count(cards);
    println!("{:?}", total);
}
