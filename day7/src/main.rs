use std::fs;

fn main() {
    let combinations: Vec<(u64, Vec<u64>)> = fs::read_to_string("res/input")
        .unwrap()
        .lines()
        .map(|l| {
            let (result_str, operands_str) = l.split_once(":").unwrap();
            let result: u64 = result_str.parse().unwrap();
            let operands: Vec<u64> = operands_str
                .split(" ")
                .filter_map(|op| match op.parse::<u64>() { Ok(x) => Some(x), Err(_) => None,
                })
                .collect();

            (result, operands)
        })
        .collect();

    let sum: u64 = combinations
        .iter()
        .filter_map(|(total, operands)| {
            if verify(*total, &operands) {
                return Some(total);
            }
            None
        })
        .sum();

    println!("{sum}");
}

fn next_ten(a: u64) -> u64 {
    if a < 10 {
        10
    } else if a < 100 {
        100
    } else {
        1000
    }
}

fn verify(total: u64, operands: &[u64]) -> bool {
    if operands.len() == 1 {
        return total == operands[0];
    }
    // println!("{:?}", operands);

    let mut mul: bool = false;
    let mut add: bool = false;
    let mut concat: bool = false;

    let operand = operands[operands.len() - 1];

    if total % operand == 0 {
        mul =  verify(total / operand, &operands[..(operands.len() - 1)]);
    } 

    if total >= operand {
        add =  verify(total - operand, &operands[..(operands.len() - 1)]);
    } 

    if total >= operand && (total - operand) % next_ten(operand) == 0 {
        concat = verify((total - operand) / next_ten(operand), &operands[..(operands.len() - 1)]);
    }

    return mul || add || concat;
}
