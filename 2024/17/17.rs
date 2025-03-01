use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::time::Instant;

#[derive(Debug)]
struct Puzzle {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    program: Vec<u32>,
    output: Vec<u32>,
}


impl Puzzle {
    fn new(reg_a: u32, reg_b: u32, reg_c: u32, program: Vec<u32>) -> Puzzle {
        let output: Vec<u32> = Vec::new();
        Puzzle {reg_a, reg_b, reg_c, program, output}
    }

    fn adv(&mut self, c_operand: u32) {
        let mut divisor: u32 = 1;
        if c_operand <= 3 {
            divisor = 2_u32.pow(c_operand as u32);
        } else {
            if c_operand == 4 {
                divisor = 2_u32.pow(self.reg_a as u32);
            } else if c_operand == 5{
                divisor = 2_u32.pow(self.reg_b as u32);
            } else if c_operand == 6 {
                divisor = 2_u32.pow(self.reg_c as u32);
            }
        }
        self.reg_a = self.reg_a / divisor;
    }

    fn bxl(&mut self, l_operand: u32) {
        self.reg_b = self.reg_b ^ l_operand;
    }

    fn bst(&mut self, c_operand: u32) {
        let mut val = c_operand;
        if val == 4 {
            val = self.reg_a;
        } else if val == 5 {
            val = self.reg_b;
        } else if val == 6 {
            val = self.reg_c;
        }
        val = val % 8;
        self.reg_b = val;
    }

    fn jnz(&self, l_operand: u32) -> i64{
        if self.reg_a == 0 {
            return -1;
        }
        l_operand as i64
    }

    fn bxc(&mut self) {
        self.reg_b = self.reg_b ^ self.reg_a;
    }

    fn out(&mut self, c_operand: u32) {
        let mut val = c_operand;
        if val == 4 {
            val = self.reg_a;
        } else if val == 5 {
            val = self.reg_b;
        } else if val == 6 {
            val = self.reg_c;
        }
        val = val % 8;
        self.output.push(val);
    }

    fn bdv(&mut self, c_operand: u32) {
        let mut divisor: u32 = 1;
        if c_operand <= 3 {
            divisor = 2_u32.pow(c_operand as u32);
        } else {
            if c_operand == 4 {
                divisor = 2_u32.pow(self.reg_a as u32);
            } else if c_operand == 5{
                divisor = 2_u32.pow(self.reg_b as u32);
            } else if c_operand == 6 {
                divisor = 2_u32.pow(self.reg_c as u32);
            }
        }
        self.reg_b = self.reg_a / divisor;
    }

    fn cdv(&mut self, c_operand: u32) {
        let mut divisor: u32 = 1;
        if c_operand <= 3 {
            divisor = 2_u32.pow(c_operand as u32);
        } else {
            if c_operand == 4 {
                divisor = 2_u32.pow(self.reg_a as u32);
            } else if c_operand == 5{
                divisor = 2_u32.pow(self.reg_b as u32);
            } else if c_operand == 6 {
                divisor = 2_u32.pow(self.reg_c as u32);
            }
        }
        self.reg_c = self.reg_a / divisor;
    }
}
fn parse_file(file_path: &str) -> (u32, u32, u32, Vec<u32>) {
    let path = Path::new(file_path);
    let file = File::open(path).expect("Failed to open the file");
    let reader = BufReader::new(file);
    
    let mut reg_a = 0;
    let mut reg_b = 0;
    let mut reg_c = 0;
    let mut program = Vec::new();
    
    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            let line_string = line.expect("Failed to read line");
            let parts: Vec<&str> = line_string.trim().split(":").collect();
            reg_a = parts[1].trim().parse::<u32>().expect("Invalid number");
        } else if i == 1 {
            let line_string = line.expect("Failed to read line");
            let parts: Vec<&str> = line_string.trim().split(":").collect();
            reg_b = parts[1].trim().parse::<u32>().expect("Invalid number");
        } else if i == 2 {
            let line_string = line.expect("Failed to read line");
            let parts: Vec<&str> = line_string.trim().split(":").collect();
            reg_c = parts[1].trim().parse::<u32>().expect("Invalid number");
        } else if i == 4 {
            let line_string = line.expect("Failed to read line");
            let parts: Vec<&str> = line_string.trim().split(":").collect();
            program = parts[1]
                .trim()
                .split(",")
                .map(|num| num.parse::<u32>().expect("Invalid number"))
                .collect();
        }
    }
    
    (reg_a, reg_b, reg_c, program)
}

fn main() {
    let file_path = "/home/ishvalin/personal/AoC/2024/17/17.txt";
    let input = parse_file(file_path);
    let mut comp = Puzzle::new(input.0, input.1, input.2, input.3);
    println!("{:?}", comp);
    let n_instructions = comp.program.len(); 
    let mut i = 0;

    while i < n_instructions {
        let opcode = comp.program[i]; 
        if i + 1 >= n_instructions {
            break; 
        }
        let operand = comp.program[i + 1];

        match opcode {
            0 => comp.adv(operand),
            1 => comp.bxl(operand),
            2 => comp.bst(operand),
            3 => {
                let a = comp.jnz(operand);
                if a != -1 {
                    i = a as usize;
                    continue;
                }
            }
            4 => comp.bxc(),
            5 => comp.out(operand),
            6 => comp.bdv(operand),
            _ => comp.cdv(operand),
        }
        println!("{:?}", comp);
        i += 2; 
    }
    let output = comp.output;
    let result = output.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");
    println!("Result: {}", result);
}
