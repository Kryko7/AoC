use std::{fs, usize};

use z3::ast::{Ast, Int};
use z3::{Config, Context, Optimize, SatResult};

#[derive(Debug)]
struct Problem {
    lights: Vec<u32>,
    buttons: Vec<Vec<u32>>,
    joltages: Vec<u32>,
}

fn parsing(filename: &str) -> Vec<Problem> {
    let content = fs::read_to_string(filename)
        .expect("Failed to read file");
    
    let output: Vec<Problem> = content
        .lines()
        .map(|s| {
            let (raw_lights, rest) = s.split_once(' ').unwrap();
            let raw_lights_char: Vec<char> = raw_lights.chars().collect();
            let mut lights :Vec<u32> = Vec::new();
            for c in raw_lights_char.iter() {
                if *c == '[' || *c == ']' {
                    continue;
                }
                if *c == '.' {
                    lights.push(0);
                } else {
                    lights.push(1);
                }
            }
            let (raw_buttons_string, raw_joltage) = rest.split_once('{').unwrap();
            let raw_buttons: Vec<Vec<u32>> = raw_buttons_string
                .trim_end()
                .split_whitespace()
                .map(|s| {
                    let a =s.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
                    let b: Vec<u32> = a.split(',').map(|p| p.parse().unwrap()).collect();
                    b
                })
                .collect();
            let n = lights.len();
            let mut buttons: Vec<Vec<u32>> = Vec::new();
            for raw_button in raw_buttons.iter() {
                let mut button: Vec<u32> = vec![0; n];
                for &b in raw_button {
                    button[b as usize] = 1;
                }
                buttons.push(button);
            }
            let raw_joltages = &raw_joltage[..raw_joltage.len()-1];
            let joltages: Vec<u32> = raw_joltages
                .split(',')
                .map(|q| {
                    q.parse().unwrap()
                })
                .collect();
            Problem { lights, buttons , joltages }
        })
        .collect();
    output
}

fn fewest_presses(problems: &Vec<Problem>) -> usize{
    let mut total_presses = 0;
    for problem in problems {
        let n = problem.lights.len();
        let mut result = vec![0u32; n];
        let mut best_press: usize = usize::MAX;
        backtrack(0, &mut result, &problem.buttons, 0, &mut best_press, &problem.lights);
        total_presses += best_press;
    }
    total_presses
}

fn fewest_presses2(problems: &Vec<Problem>) -> usize{
    let mut total_presses = 0;
    for problem in problems {
        let n = problem.lights.len();
        let mut result = vec![0u32; n];
        let mut best_press: usize = usize::MAX;
        backtrack2(0, &mut result, &problem.buttons, 0, &mut best_press, &problem.joltages);
        total_presses += best_press;
    }
    total_presses
}

fn backtrack(
    btn_idx: usize,
    current_lights: &mut Vec<u32>,
    buttons: &Vec<Vec<u32>>,
    current_press_count: usize,
    best_pres: &mut usize,
    lights: &Vec<u32>,
) {
    if current_press_count >= *best_pres {
        return;
    }

    if btn_idx == buttons.len() {
        let n = current_lights.len();
        for i in 0..n {
            if lights[i] != current_lights[i] % 2 {
                return;
            }
        }
        *best_pres = (*best_pres).min(current_press_count);
        return;
    }

    backtrack(btn_idx + 1, current_lights, buttons, current_press_count, best_pres, lights);

    for i in 0..lights.len() {
        current_lights[i] += buttons[btn_idx][i];
    }

    backtrack(btn_idx + 1, current_lights, buttons, current_press_count + 1, best_pres, lights);

    for i in 0..lights.len() {
        current_lights[i] -= buttons[btn_idx][i];
    }

}


fn backtrack2(
    btn_idx: usize,
    current_joltages: &mut Vec<u32>,
    buttons: &Vec<Vec<u32>>,
    current_press_count: usize,
    best_pres: &mut usize,
    joltages: &Vec<u32>,
) {
    if current_press_count >= *best_pres {
        return;
    }

    if btn_idx == buttons.len() {
        let n = current_joltages.len();
        for i in 0..n {
            if joltages[i] != current_joltages[i]{
                return;
            }
        }
        *best_pres = (*best_pres).min(current_press_count);
        return;
    }

    for i in 0..joltages.len() {
        if current_joltages[i] > joltages[i] {
            return;
        }
    }

    backtrack2(btn_idx + 1, current_joltages, buttons, current_press_count, best_pres, joltages);

    for i in 0..joltages.len() {
        current_joltages[i] += buttons[btn_idx][i];
    }

    backtrack2(btn_idx, current_joltages, buttons, current_press_count + 1, best_pres, joltages);

    for i in 0..joltages.len() {
        current_joltages[i] -= buttons[btn_idx][i];
    }

}

fn part2_with_z3(problems: &Vec<Problem>) -> u32 {
    let mut total_presses = 0;
    for problem in problems {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);

        let opt = Optimize::new(&ctx);
        let n_buttons = problem.buttons.len();
        let mut btn_vars: Vec<Int> = Vec::with_capacity(n_buttons);
        for i in 0..n_buttons {
            let name = format!("btn_{}", i);
            let var = Int::new_const(&ctx, name.as_str());

            opt.assert(&var.ge(&Int::from_i64(&ctx, 0)));
            
            btn_vars.push(var);
        }
        for i in 0..problem.joltages.len() {
            let terms: Vec<Int> = (0..n_buttons)
                .map(|j| {
                    &btn_vars[j] * Int::from_i64(&ctx, problem.buttons[j][i].into())
                })
                .collect();

            let constraint_eq = Int::add(&ctx, &terms.iter().collect::<Vec<_>>());

            opt.assert(
                &constraint_eq._eq(&Int::from_i64(&ctx, problem.joltages[i].into()))
            );
        }

        let sum_btns = Int::add(
            &ctx,
            &btn_vars.iter().collect::<Vec<_>>(),
        );

        opt.minimize(&sum_btns);

        if opt.check(&[]) == SatResult::Sat {
            let model = opt.get_model().unwrap();

            let presses: i64 = btn_vars.iter()
                .map(|v| model.eval(v, true).unwrap().as_i64().unwrap())
                .sum();

            total_presses += presses as u32;
        }
    }
    total_presses
}
fn main() {
    let filename = "/home/minindu/personal/AoC/rust_proj/src/input.txt";
    let problems = parsing(filename);
    let minimum_presses: usize = fewest_presses(&problems);
    println!("{:?}", minimum_presses);
    let minimum_presses2: u32 = part2_with_z3(&problems);
    println!("{:?}", minimum_presses2)
}