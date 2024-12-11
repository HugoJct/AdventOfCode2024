use std::{collections::HashMap, fs};

fn main() {
    println!("Hello, world!");

    let mut times: HashMap<u64, u64> = HashMap::new();

    fs::read_to_string("res/input")
        .unwrap()
        .lines()
        .for_each(|l| {
            l.split(" ").into_iter().for_each(|c| {
                *times.entry(c.parse().unwrap()).or_insert(0) += 1;
            });
        });

    for _ in 0..75 {
        let mut new_map: HashMap<u64, u64> = HashMap::new();

        for (&k, &v) in &times {
            if k == 0 {
                *new_map.entry(1).or_insert(0) += v;
            } else if k.to_string().len() % 2 == 0 {
                let istr = k.to_string();
                *new_map.entry(istr[..istr.len() / 2].parse::<u64>().unwrap()).or_insert(0) += v;
                *new_map.entry(istr[(istr.len() / 2)..].parse::<u64>().unwrap()).or_insert(0) += v;
            } else {
                *new_map.entry(k * 2024).or_insert(0) += v;
            }
        }
        times = new_map;
    }

    println!("{}", times.values().sum::<u64>());
}
