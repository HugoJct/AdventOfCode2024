use std::{fs, str::FromStr};

#[derive(Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

struct Guard {
    direction: Direction,
    position: (usize, usize),
}

impl Guard {
    fn new() -> Self {
        Guard {
            direction: Direction::UP,
            position: (0, 0),
        }
    }

    fn set_pos(&mut self, newx: usize, newy: usize) {
        self.position.0 = newx;
        self.position.1 = newy;
    }

    fn walk(&mut self, grid: &mut Vec<Vec<char>>) {
        let (x, y) = self.position;
        match self.direction {
            Direction::UP => {
                grid[x][y] = 'X';
                if grid[x-1][y] == '#' {
                    self.turn();
                } else {
                    self.set_pos(x-1, y);
                }
            }
            Direction::RIGHT => {
                grid[x][y] = 'X';
                if grid[x][y+1] == '#' {
                    self.turn();
                } else {
                    self.set_pos(x , y+1);
                }
            }
            Direction::DOWN => {
                grid[x][y] = 'X';
                if grid[x+1][y ] == '#' {
                    self.turn();
                } else {
                    self.set_pos(x+1, y);
                }
            }
            Direction::LEFT => {
                grid[x][y] = 'X';
                if grid[x ][y-1] == '#' {
                    self.turn();
                } else {
                    self.set_pos(x , y-1);
                }
            }
        }
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        };
    }
}

fn main() {
    println!("Hello, world!");

    let mut guard: Guard = Guard::new();

    let input: Vec<String> = fs::read_to_string("res/test")
        .unwrap()
        .lines()
        .map(|l| String::from_str(l).unwrap())
        .collect();

    let mut grid: Vec<Vec<char>> = vec![vec!['*'; input.len() + 2]; 1];

    grid.append(
        &mut input
            .into_iter()
            .enumerate()
            .map(|(index, line)| match line.find(|x| x == '^') {
                Some(i) => {
                    guard.set_pos(index + 1, i + 1);
                    let mut a = vec!['*'; 1];
                    a.append(&mut line.replace("^", ".").chars().collect::<Vec<char>>());
                    a.push('*');
                    a
                }
                None => {
                    let mut a = vec!['*'; 1];
                    a.append(&mut line.chars().collect::<Vec<char>>());
                    a.push('*');
                    a
                }
            })
            .collect(),
    );
    grid.push(grid[0].clone());

    while grid[guard.position.0 as usize][guard.position.1 as usize] != '*' {
        // for x in 0..grid.len() {
        //     for y in 0..grid[x].len() {
        //         if x == guard.position.0 && y == guard.position.1 {
        //             match guard.direction {
        //                 Direction::UP => print!("^ "),
        //                 Direction::RIGHT => print!("> "),
        //                 Direction::LEFT => print!("< "),
        //                 Direction::DOWN => print!("v "),
        //             }
        //         } else {
        //             print!("{} ", grid[x][y]);
        //         }
        //     }
        //     println!("");
        // }

        // println!("{} {}", guard.position.0, guard.position.1);
        // thread::sleep(Duration::from_millis(500));
        guard.walk(&mut grid);
    }
    // for l in &grid {
    //     println!("{:?}", l);
    // }
    println!(
        "{}",
        grid.into_iter()
            .map(|l| l.into_iter().filter(|c| *c == 'X').count() as u32)
            .sum::<u32>()
    )
}
