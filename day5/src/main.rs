use std::{cmp::Ordering, fs};

fn main() {
    println!("Hello, world!");
    let input = fs::read_to_string("res/input").unwrap();
    let (rules, lists) = input.split_once("\n\n").unwrap();

    let mut comps: Vec<Vec<Ordering>> = vec![vec![Ordering::Equal; 100]; 100];

    for rule in rules.lines() {
        let (x, y) = rule.split_once("|").unwrap();
        let sm: usize = x.parse().unwrap();
        let bg: usize = y.parse().unwrap();
        comps[sm][bg] = Ordering::Less;
        comps[bg][sm] = Ordering::Greater;
    }

    let mut total: u32 = 0;

    for list in lists.lines() {
        let numbers : Vec<usize> = list.split(",").filter_map(|x| match x.parse::<usize>() {
            Ok(x) => Some(x),
            Err(_) => None,
        }).collect();
        let mut numbers_sorted: Vec<usize> = numbers.clone();
        numbers_sorted.sort_by(|x, y| comps[*x][*y]);

        let mut sorted: bool = true;
        for i in 0..numbers_sorted.len() {
            if numbers[i] != numbers_sorted[i] {
                sorted = false;
                break;
            }
        }
        if sorted {
            println!("{:?}", numbers);
            total += numbers[numbers.len() / 2] as u32;
        }
    }

    println!("{}", total);
}
