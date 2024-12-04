use std::fs;

use diagonal::straight_y;
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

    let regex_first_third_line = Regex::new(r"^M.S").unwrap();
    let regex_second_line = Regex::new(r"^.A.").unwrap();

    let regex_first_third_line_rev = Regex::new(r"^S.M").unwrap();

    let mut total: u32 = 0;

    input
        .clone()
        .into_iter()
        .zip(input.clone().into_iter().skip(1))
        .zip(input.clone().into_iter().skip(2))
        .for_each(|((one, two), three)| {
            for i in 0..one.len() {
                if regex_first_third_line.is_match(&one[i..])
                    && regex_second_line.is_match(&two[i..])
                    && regex_first_third_line.is_match(&three[i..])
                {
                    total += 1;
                } else if regex_first_third_line_rev.is_match(&one[i..])
                    && regex_second_line.is_match(&two[i..])
                    && regex_first_third_line_rev.is_match(&three[i..])
                {
                    total += 1;
                }
            }
        });

    input_rev
        .clone()
        .into_iter()
        .zip(input_rev.clone().into_iter().skip(1))
        .zip(input_rev.clone().into_iter().skip(2))
        .for_each(|((one, two), three)| {
            for i in 0..one.len() {
                if regex_first_third_line.is_match(&one[i..])
                    && regex_second_line.is_match(&two[i..])
                    && regex_first_third_line.is_match(&three[i..])
                {
                    total += 1;
                } else if regex_first_third_line_rev.is_match(&one[i..])
                    && regex_second_line.is_match(&two[i..])
                    && regex_first_third_line_rev.is_match(&three[i..])
                {
                    total += 1;
                }
            }
        });

    println!("{}", total);
}
