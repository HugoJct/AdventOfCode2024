use std::fs;

fn ways_to_top(grid: &Vec<Vec<u32>>, start: (usize, usize)) -> u32 {
    let (x, y) = start;

    if grid[x][y] == 9 {
        return 1;
    }

    let mut left_count: u32 = 0;
    let mut right_count: u32 = 0;
    let mut bot_count: u32 = 0;
    let mut top_count: u32 = 0;

    if x > 0 && grid[x - 1][y] == grid[x][y] + 1 {
        left_count = ways_to_top(grid, ((x - 1), y))
    }

    if x < grid.len() - 1 && grid[x + 1][y] == grid[x][y] + 1 {
        right_count = ways_to_top(grid, ((x + 1), y))
    }

    if y > 0 && grid[x][y - 1] == grid[x][y] + 1 {
        top_count = ways_to_top(grid, (x, (y - 1)))
    }

    if y < grid.len() - 1 && grid[x][y + 1] == grid[x][y] + 1 {
        bot_count = ways_to_top(grid, (x, (y + 1)))
    }

    left_count + right_count + bot_count + top_count
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
                let a = ways_to_top(&input, (i, j));
                total += a;
            }
        }
    }

    println!("{}", total);
}
