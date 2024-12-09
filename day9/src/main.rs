use std::fs;

fn find_space(chars: &Vec<u32>, size: usize, before: usize) -> Option<usize> {
    let mut i: usize = 0;

    let mut start: usize = 0;
    let mut size_counted: usize = 0;

    while i < chars.len() {
        if chars[i] == '.' as u32 {
            if size_counted == 0 {
                start = i;
            }
            size_counted += 1;
        } else {
            if size_counted >= size {
                break;
            } else {
                start = 0;
                size_counted = 0;
            }
        }
        i += 1;
    }

    if start > 0 && start < before {
        return Some(start);
    }
    None
}

fn main() {
    println!("Hello, world!");
    let mut empty: bool = false;
    let mut next_id: u32 = ('.' as u32) + 2; // not set to 0 to avoid ASCII 0x2E to be
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

    let mut size: u32 = 0;
    let mut file: i32 = -1;
    for i in (0..input.len()).rev() {
        if input[i] != file as u32 && size != 0 {
            let index = find_space(&input, size as usize, i);
            match index {
                Some(x) => {
                    for j in x..(x + (size as usize)) {
                        input[j] = file as u32;
                    }
                    for j in (i+1)..(i + (size as usize) + 1) {
                        input[j] = '.' as u32;
                    }
                }
                None => {}
            }

            if input[i] == '.' as u32 {
                size = 0;
                file = -1;
            } else {
                size = 1;
                file = input[i] as i32;
            }
            continue;
        } else {
            if file == -1 {
                file = input[i] as i32;
                size += 1;
            } else if input[i] == (file as u32) {
                size += 1;
            }
        }
    }

    // for c in &input {
    //     print!("{}", char::from_u32(*c).unwrap());
    // }
    // println!("");

    println!(
        "{}",
        input
            .iter()
            .enumerate()
            .fold(0_u64, |acc, (index, val)| {
                if *val == '.' as u32 {
                    return acc;
                }
                acc + (((*val as u32) - (('.' as u32) + 2)) as u64 * (index as u64))
            })
    );
}
