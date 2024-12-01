use std::fs;

fn main() {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    fs::read_to_string("res/input")
        .unwrap()
        .lines()
        .for_each(|e| {
            let columns = e.split_whitespace();
            for (index, column) in columns.enumerate() {
                if index == 0 {
                    left.push(column.to_string().parse().unwrap());
                } else {
                    right.push(column.to_string().parse().unwrap());
                }
            }
        });

    let first_part = first_part(&mut left, &mut right);
    let second_part = second_part(&mut left, &mut right);

    println!("{}", first_part);
    println!("{}", second_part);
}

fn first_part(left: &mut Vec<u32>, right: &mut Vec<u32>) -> u32 {
    let mut total: u32 = 0;

    left.sort();
    right.sort();

    for (l, r) in left.into_iter().zip(right.into_iter()) {
        total += l.abs_diff(*r);
    }

    total
}

fn second_part(left: &mut Vec<u32>, right: &mut Vec<u32>) -> u32 {
    let mut total: u32 = 0;

    left.sort();
    right.sort();

    for l in left.into_iter() {
        let mut current : u32 = 0;
        for r in right.into_iter() {
            if r == l {
               current += 1; 
            }
        }
        total += *l * current;
    }

    total
}

