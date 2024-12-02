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
        .map(|x| {
            if only_increasing(&x) || only_decreasing(&x) {
                return true;
            }

            for (i, _) in x.clone().into_iter().enumerate() {       // test the array again by
                // removing all the element one by one to find a working combination
                let mut tmp = x.clone();
                tmp.remove(i);
                if only_increasing(&tmp) || only_decreasing(&tmp) {
                    return true;
                }
            }
            false
        })
        .filter(|x| x == &true)
        .count();

    println!("{count}");
}

fn only_increasing(line: &Vec<i32>) -> bool {
    !line
        .clone()
        .into_iter()
        .zip(line.clone().into_iter().skip(1))
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
