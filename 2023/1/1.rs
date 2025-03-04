use std::{fs::File, io::{BufRead, BufReader}, path::Path};

fn parser(filepath: &str) -> Vec<String> {
    let path = Path::new(filepath);
    let file = File::open(path).expect("Failed to open the file");
    let reader = BufReader::new(file);

    let mut parsed_string: Vec<String> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let line_string = line.expect("Failed to read line");
        parsed_string.push(line_string);
    }
    parsed_string
}


fn get_calibration_value(value: String) -> i32 {
    let mut first_digit = '0';
    let mut second_digit: char = '0';
    let mut first_found = false;
    for c in value.chars() {
        if c.is_ascii_digit() {
            if !first_found {
                first_digit = c;
                first_found = true;
            }
            if first_found {
                second_digit = c;
            }
        }
    }
    let mut s = String::new();
    s.push(first_digit);
    s.push(second_digit);
    s.to_string().parse::<i32>().unwrap()
}

fn main() {
    let filepath = "/home/ishvalin/personal/AoC/2023/1/1.txt";
    let parsed_strings = parser(filepath);
    let mut cal_sum = 0;
    for v in parsed_strings {
        let val = get_calibration_value(v);
        cal_sum += val;
    }
    println!("Sum of calibarion values: {cal_sum}")
}



