use std::fs;

use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Multiply(u32, u32),
}

fn main() {
    let mut enabled: bool = true;
    let total: u32 = fs::read_to_string("res/input")
        .unwrap()
        .lines()
        .map(|line| {
            Regex::new(r"(do|don't|mul)\(((?:[0-9]+,[0-9]+)?)\)")
                .unwrap()
                .captures_iter(line)
                .filter_map(|mm| {
                    let (_, [instr, value]) = mm.extract();
                    match instr {
                        "mul" => {
                            let (x, y) = value.split_once(",").unwrap();
                            Some(Instruction::Multiply(
                                x.parse().unwrap(),
                                y.parse().unwrap(),
                            ))
                        }
                        "do" => Some(Instruction::Do),
                        "don't" => Some(Instruction::Dont),
                        _ => None,
                    }
                })
                .filter_map(|instr| match instr {
                    Instruction::Multiply(x, y) => {
                        if enabled {
                            Some(x * y)
                        } else {
                            None
                        }
                    }
                    Instruction::Do => {
                        enabled = true;
                        None
                    }
                    Instruction::Dont => {
                        enabled = false;
                        None
                    }
                })
                .sum::<u32>()
        })
        .sum();
    println!("{}", total);
}
