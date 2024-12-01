use std::fs;

fn main() {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    fs::read_to_string("res/input")
        .unwrap()
        .lines()
        .for_each(|e| {
            e.split_whitespace().enumerate().for_each(|(index, column)| {
                if index == 0 {
                    left.push(column.to_string().parse().unwrap());
                } else {
                    right.push(column.to_string().parse().unwrap());
                }
            });
        });

    let first_part = first_part(&mut left, &mut right);
    let second_part = second_part(&left, &right);

    println!("{}", first_part);
    println!("{}", second_part);
}

fn first_part(left: &mut Vec<u32>, right: &mut Vec<u32>) -> u32 {
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right.into_iter())
        .fold(0, |acc, (l, r)| acc + l.abs_diff(*r))
}

fn second_part(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    left.into_iter()
        .map(|l| *l * (right.into_iter().filter(|x| *x == l).count() as u32))
        .fold(0, |acc, num| acc + num)
}
