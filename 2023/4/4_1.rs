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
    for card in cards {
        let mut count:u32 = 0;
        for num in card.owned_cards {
            if card.winning_cards.contains(&num) {
                count += 1;
            }
        }
        if count >= 1 {
            total += (2_u32).pow(count-1);
        }
    }
    total
}


fn main() {
    let file_path = "/home/ishvalin/personal/AoC/2023/4/4.txt";
    let cards:Vec<Card> = parsing(file_path);
    let total = count(cards);
    println!("{:?}", total);
}
