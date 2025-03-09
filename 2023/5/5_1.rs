use std::{fs::File, io::{BufRead, BufReader}, path::Path};

#[derive(Debug)]
struct Mappping {
    seeds: Vec<u64>,
    seeds_to_soil: Vec<Range>,
    soil_to_fertilizer: Vec<Range>,
    fertilizer_to_water: Vec<Range>,
    water_to_light: Vec<Range>,
    light_to_temp: Vec<Range>,
    temp_to_humidity: Vec<Range>,
    humidity_to_location: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    source_start: u64,
    destination_start:u64,
    range: u64,
}

fn parsing(filename: &str) -> Mappping {
    let path = Path::new(filename);
    let file = File::open(path).expect("Failed to open the file.");

    let reader = BufReader::new(file);
    let mut i = 0;
    let mut skip = false;

    let mut seeds: Vec<u64> = Vec::new();
    let mut seeds_to_soil: Vec<Range> = Vec::new();
    let mut soil_to_fertilizer: Vec<Range> = Vec::new();
    let mut fertilizer_to_water:Vec<Range> = Vec::new();
    let mut water_to_light: Vec<Range> = Vec::new();
    let mut light_to_temp: Vec<Range> = Vec::new();
    let mut temp_to_humidity: Vec<Range> = Vec::new();
    let mut humidity_to_location: Vec<Range>= Vec::new();

    for (_, line) in reader.lines().enumerate() {
        if skip {
            skip = false;
            continue;
        }
        let line_string = line.expect("Failed to read line").trim().to_string();
        if line_string.is_empty() {
            i += 1;
            skip = true;
            continue;
        }
        match i {
            0 => {
                let parts: Vec<&str> = line_string.split(":").collect();
                seeds = parts[1]
                                .trim()
                                .split_whitespace()
                                .map(|num| num.parse::<u64>().expect("Invalid number"))
                                .collect();
            }
            1 => {
                let nums: Vec<u64> = line_string
                                                    .trim()
                                                    .split_whitespace()
                                                    .map(|num| num.parse::<u64>().expect("Invalid number"))
                                                    .collect();
                seeds_to_soil.push(Range { source_start: nums[1], destination_start: nums[0], range: nums[2] });
            }
            2 => {
                let nums: Vec<u64> = line_string
                                                    .trim()
                                                    .split_whitespace()
                                                    .map(|num| num.parse::<u64>().expect("Invalid number"))
                                                    .collect();
                
                soil_to_fertilizer.push(Range { source_start: nums[1], destination_start: nums[0], range: nums[2] });
                
            }
            3 => {
                let nums: Vec<u64> = line_string
                                                    .trim()
                                                    .split_whitespace()
                                                    .map(|num| num.parse::<u64>().expect("Invalid number"))
                                                    .collect();
                
                fertilizer_to_water.push(Range { source_start: nums[1], destination_start: nums[0], range: nums[2] });
                
            }
            4 => {
                let nums: Vec<u64> = line_string
                                                    .trim()
                                                    .split_whitespace()
                                                    .map(|num| num.parse::<u64>().expect("Invalid number"))
                                                    .collect();

                water_to_light.push(Range { source_start: nums[1], destination_start: nums[0], range: nums[2] });
                
            }
            5 => {
                let nums: Vec<u64> = line_string
                                                    .trim()
                                                    .split_whitespace()
                                                    .map(|num| num.parse::<u64>().expect("Invalid number"))
                                                    .collect();

                light_to_temp.push(Range { source_start: nums[1], destination_start: nums[0], range: nums[2] });

            }
            6 => {
                let nums: Vec<u64> = line_string
                                                    .trim()
                                                    .split_whitespace()
                                                    .map(|num| num.parse::<u64>().expect("Invalid number"))
                                                    .collect();
                temp_to_humidity.push(Range { source_start: nums[1], destination_start: nums[0], range: nums[2] });
                
            }
            7 => {
                let nums: Vec<u64> = line_string
                                                    .trim()
                                                    .split_whitespace()
                                                    .map(|num| num.parse::<u64>().expect("Invalid number"))
                                                    .collect();
                humidity_to_location.push(Range { source_start: nums[1], destination_start: nums[0], range: nums[2] });
                
            }
            _ => continue,
        }
    }
    Mappping{ seeds, seeds_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temp, temp_to_humidity, humidity_to_location}
}

fn find_loc(mapping: Mappping) -> Vec<u64>{
    let mut locations: Vec<u64> = Vec::new();
    for seed in mapping.seeds {
        let soil = find_a_destination(&mapping.seeds_to_soil, seed);
        let fertilzer = find_a_destination(&mapping.soil_to_fertilizer, soil);
        let water =find_a_destination(&mapping.fertilizer_to_water, fertilzer);
        let light = find_a_destination(&mapping.water_to_light, water);
        let temp = find_a_destination(&mapping.light_to_temp, light);
        let humidity = find_a_destination(&mapping.temp_to_humidity, temp);
        let location = find_a_destination(&mapping.humidity_to_location, humidity);
        locations.push(location);
    }
    locations
}


fn find_a_destination(range: &Vec<Range>, source: u64) -> u64 {
    for r in range {
        if source >= r.source_start && source < r.source_start+r.range {
            return r.destination_start+(source-r.source_start);
        }
    }
    source
}

fn main() {
    let filename = "/home/ishvalin/personal/AoC/aoc/src/text.txt";
    let mapping  = parsing(filename);
    let locations = find_loc(mapping);

    if let Some(min_value) = locations.iter().min() {
        println!("The minimum value location is {}", min_value);
    } else {
        println!("Failed to find the minimum");
    }
}