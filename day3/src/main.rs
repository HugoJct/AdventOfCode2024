use std::fs;

use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Multiply(u32, u32),
}

fn main() {
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
                .map(|instr| match instr {
                    Instruction::Multiply(x, y) => x * y,
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum();
    println!("{}", total);
}
