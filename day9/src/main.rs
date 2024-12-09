use std::fs;

fn main() {
    println!("Hello, world!");
    let mut empty: bool = false;
    let mut next_id: u32 = ('.' as u32) + 1; // not set to set to 0 to avoid ASCII 0x 2E to be
                                             // interpreted as '.'
    let mut input: Vec<u32> = fs::read_to_string("res/input")
        .unwrap()
        .chars()
        .fold(String::new(), |acc, c| {
            if c == '\n' {
                return acc;
            }

            let size: usize = usize::try_from(c.to_digit(10).unwrap()).unwrap();
            if empty == false {
                empty = true;
                let blocks = (0..size)
                    .map(|_| char::from_u32(next_id).unwrap())
                    .collect::<String>();
                next_id += 1;
                return acc + &blocks;
            } else {
                empty = false;
                return acc + &(0..size).map(|_| '.').collect::<String>();
            }
        })
        .chars()
        .map(|c| c as u32)
        .collect();

    let mut start: usize = 0;
    let mut end: usize = input.len() - 1;

    while start <= end {
        if input[start] == '.' as u32 {
            while input[end] == '.' as u32 {
                end -= 1;
            }
            input[start] = input[end];
            input[end] = '.' as u32;
            end -= 1;
        }
        start += 1;
    }

    println!(
        "{}",
        input[..input.iter().position(|c| *c == '.' as u32).unwrap()]
            .iter()
            .enumerate()
            .fold(0_u64, |acc, (index, val)| {
                acc + (((*val as u32) - (('.' as u32) + 1)) as u64 * (index as u64))
            })
    );
}
