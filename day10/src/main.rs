use std::fs;

fn ways_to_top(grid: &Vec<Vec<u32>>, start: (usize, usize), acc: &mut Vec<(usize, usize)>) {
    let (x, y) = start;

    if grid[x][y] == 9 && acc.contains(&(x, y)) == false {
        acc.push((x, y));
    }

    if x > 0 && grid[x - 1][y] == grid[x][y] + 1 {
        ways_to_top(grid, ((x - 1), y), acc)
    }

    if x < grid.len() - 1 && grid[x + 1][y] == grid[x][y] + 1 {
        ways_to_top(grid, ((x + 1), y), acc)
    }

    if y > 0 && grid[x][y - 1] == grid[x][y] + 1 {
        ways_to_top(grid, (x, (y - 1)), acc)
    }

    if y < grid.len() - 1 && grid[x][y + 1] == grid[x][y] + 1 {
        ways_to_top(grid, (x, (y + 1)), acc);
    }
}

fn tops_reachable(grid: &Vec<Vec<u32>>, start: (usize, usize)) -> u32 {
    let mut acc: Vec<(usize, usize)> = vec![];
    ways_to_top(grid, start, &mut acc);
    acc.len() as u32
}

fn main() {
    println!("Hello, world!");

    let input: Vec<Vec<u32>> = fs::read_to_string("res/input")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut total: u32 = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 0 {
                let a = tops_reachable(&input, (i, j));
                total += a;
            }
        }
    }
    println!("{}", total);
}
