use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use regex::Regex;


fn parse_file(file_path: &str) -> Vec<Vec<i32>> {
    let mut robots: Vec<Vec<i32>> = Vec::new();
    let path = Path::new(file_path);

    let file = File::open(path).expect("Failed to open file.");
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line").trim().to_string();

        let re = Regex::new(r"([-]?\d+),([-]?\d+) v=([-]?\d+),([-]?\d+)").unwrap();

        let mut coordinates: Vec<i32> = Vec::new();
        if let Some(caps) = re.captures(&line) {
            let x_p = &caps[1];
            let y_p = &caps[2];
            let x_v = &caps[3];
            let y_v = &caps[4];
            coordinates.push(x_p.parse::<i32>().unwrap());
            coordinates.push(y_p.parse::<i32>().unwrap());
            coordinates.push(x_v.parse::<i32>().unwrap());
            coordinates.push(y_v.parse::<i32>().unwrap());
        }
        robots.push(coordinates);
    }
    robots
}

fn calc_safety(robots: &Vec<Vec<i32>>, width: i32, height: i32, mid_x: i32, mid_y: i32) ->i32 {
    let mut first_quad = 0;
    let mut second_quad = 0;
    let mut third_quad = 0;
    let mut fourth_quad = 0;

    for robot in robots {
        let mut x_p = robot[0];
        let mut y_p = robot[1];
        let x_v = robot[2];
        let y_v = robot[3];

        x_p = ((x_p + x_v * 100) % width + width) % width;
        y_p = ((y_p + y_v * 100) % height + height) % height;

       

        println!("X: {} Y: {}", x_p, y_p);
        if x_p < mid_x && y_p < mid_y {
            first_quad += 1;
        } else if x_p < mid_x && y_p > mid_y {
            second_quad += 1;
        } else if x_p > mid_x && y_p < mid_y {
            third_quad += 1;
        } else if x_p > mid_x && y_p > mid_y {
            fourth_quad += 1;
        }
    }
    // println!("{}", first_quad);
    // println!("{}", second_quad);
    // println!("{}", third_quad);
    // println!("{}", fourth_quad);
    first_quad * second_quad * third_quad * fourth_quad
}

fn main() {
    let path = "/home/ishvalin/personal/AoC/2024/14/14.txt";
    let robots = parse_file(path);
    // println!("{:?}", robots);
    let safety_factor = calc_safety(&robots, 101, 103, 50, 51);
    println!("{:?}", safety_factor);
}