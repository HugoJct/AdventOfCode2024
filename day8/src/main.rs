use std::{cmp::min, fs};

fn main() {
    println!("Hello, world!");

    let grid: Vec<Vec<char>> = fs::read_to_string("res/input")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    // for i in &grid {
    //     for j in i {
    //         print!("{j} ");
    //     }
    //     println!("");
    // }

    let mut pairs: Vec<((usize, usize), (usize, usize))> = vec![];
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] != '.' {
                let char = grid[i][j];
                for ibis in 0..grid.len() {
                    for jbis in 0..grid[i].len() {
                        if grid[ibis][jbis] == char && (ibis != i || jbis != j) {
                            if pairs.contains(&((ibis, jbis), (i, j))) == false {
                                pairs.push(((i, j), (ibis, jbis)));
                            }
                        }
                    }
                }
            }
        }
    }

    let mut grid_antinode: Vec<Vec<char>> = grid.clone();
    for ((x1, y1), (x2, y2)) in &pairs {
        // println!("{} {} - {} {}", x1, y1, x2, y2);
        let minx = min(x1, x2);
        let miny = min(y1, y2);
        let maxx = if x1 == minx { x2 } else { x1 };
        let maxy = if y1 == miny { y2 } else { y1 };

        let difx = maxx - minx;
        let dify = maxy - miny;

        // println!("difx {difx} dify {dify}");
        // println!("minx {minx} maxx {maxx}");
        // println!("miny {miny} maxy {maxy}");

        if x1 <= x2 && y1 <= y2 {
            let mut i: usize = 0;
            while *x1 >= difx * i && *y1 >= dify * i {
                grid_antinode[x1 - i * difx][y1 - i * dify] = '#';
                i+= 1;
            }

            i = 0;
            while x2 + difx * i < grid_antinode.len() && y2 + dify * i < grid_antinode.len() {
                grid_antinode[x2 + difx * i][y2 + dify * i] = '#';
                i += 1;
            }
        } else {
            let mut i: usize = 0;

            while x2 + difx * i < grid_antinode.len() && dify * i <= *y2 {
                grid_antinode[x2 + difx * i][y2 - dify * i] = '#';
                i+= 1;
            }

            i = 0;
            while y1 + dify * i < grid_antinode.len() && difx * i <= *x1 {
                grid_antinode[x1 - difx * i][y1 + dify * i] = '#';
                i+=1;
            }
        }
    }

    // println!("{:?}", pairs);

    // for i in &grid_antinode {
    //     for j in i {
    //         print!("{j} ");
    //     }
    //     println!("");
    // }
    let sum: u32 = grid_antinode
        .iter()
        .map(|l| l.iter().filter(|c| **c == '#').count() as u32)
        .sum();

    println!("{}", sum);
}
