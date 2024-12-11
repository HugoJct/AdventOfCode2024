use std::fs;

fn main() {
    println!("Hello, world!");

    let mut input: Vec<Vec<u64>> = fs::read_to_string("res/input")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(" ")
                .into_iter()
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect();

    for _ in 0..25 {
        input[0] = input[0]
            .iter()
            .flat_map(|i| {
                let mut vec: Vec<u64> = vec![];
                if *i == 0 {
                    vec.push(1)
                } else if i.to_string().len() % 2 == 0 {
                    let istr = i.to_string();
                    vec.push(istr[..istr.len() / 2].parse().unwrap());
                    vec.push(istr[(istr.len() / 2)..].parse().unwrap());
                } else {
                    vec.push(*i * 2024)
                }
                vec
            })
            .collect();

    //     for e in &input[0] {
    //         print!("{} ", e);
    //     }
    //
    // println!("");
    }
    println!("{}", input[0].len());
}
