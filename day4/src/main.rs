use std::fs;

use diagonal::{diagonal_pos_neg, diagonal_pos_pos, straight_y};
use regex::Regex;

fn main() {
    let file = fs::read_to_string("res/input").unwrap();

    let input = file.lines().collect::<Vec<_>>();

    let input_rev = straight_y(&input).into_iter().map(|l| {
        l.into_iter()
            .filter_map(|x| char::from_u32(*x as u32))
            .fold(String::from(""), |mut acc, x| {
                acc.push(x);
                acc
            })
    });

    let input_diag = diagonal_pos_pos(&input).into_iter().map(|l| {
        l.into_iter()
            .filter_map(|x| char::from_u32(*x as u32))
            .fold(String::from(""), |mut acc, x| {
                acc.push(x);
                acc
            })
    });

    let input_diag_rev = diagonal_pos_neg(&input).into_iter().map(|l| {
        l.into_iter()
            .filter_map(|x| char::from_u32(*x as u32))
            .fold(String::from(""), |mut acc, x| {
                acc.push(x);
                acc
            })
    });

    let regex = Regex::new(r"XMAS").unwrap();
    let regex_rev = Regex::new(r"SAMX").unwrap();

    let mut total: u32 = 0;

    total += input.clone().into_iter().fold(0, |acc, l| {
        acc + regex.find_iter(l).count() as u32 + regex_rev.find_iter(l).count() as u32
    });

    total += input_rev.fold(0, |acc, l| {
        acc + regex.find_iter(&l).count() as u32 + regex_rev.find_iter(&l).count() as u32
    });

    total += input_diag.fold(0, |acc, l| {
        acc + regex.find_iter(&l).count() as u32 + regex_rev.find_iter(&l).count() as u32
    });

    total += input_diag_rev.fold(0, |acc, l| {
        acc + regex.find_iter(&l).count() as u32 + regex_rev.find_iter(&l).count() as u32
    });

    println!("{}", total);
}
