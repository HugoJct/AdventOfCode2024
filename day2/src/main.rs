use std::fs;

fn main() {
    let count = fs::read_to_string("res/input")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|x| only_increasing(&x) || only_decreasing(&x))
        .filter(|x| x == &true)
        .count();

    println!("{count}");
}

fn only_increasing(line: &Vec<i32>) -> bool {
    !line
        .clone()
        .into_iter()
        .zip(line.into_iter().skip(1))
        .map(|(one, two)| {
            let diff = one - two;
            diff >= 1 && diff <= 3
        })
        .collect::<Vec<bool>>()
        .contains(&false)
}

fn only_decreasing(line: &Vec<i32>) -> bool {
    !line
        .clone()
        .into_iter()
        .zip(line.into_iter().skip(1))
        .map(|(one, two)| {
            let diff = two - one;
            diff >= 1 && diff <= 3
        })
        .collect::<Vec<bool>>()
        .contains(&false)
}
