use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use std::collections::HashMap;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Forgive me father, for I have bruteforced
fn simulate(obstacles: &Vec<Vec<bool>>, cur_x: i32, cur_y: i32) -> bool {
    let mut cur_dir: Direction = Direction::Up;
    let mut cur_x = cur_x;
    let mut cur_y = cur_y;
    let mut dir_x: i32 = -1;
    let mut dir_y: i32 = 0;
    let mut visited_cnt: HashMap<(i32, i32), i32> = HashMap::new();

    while *visited_cnt.get(&(cur_x, cur_y)).unwrap_or(&0) <= 4 {
        visited_cnt.entry((cur_x, cur_y)).and_modify(|x| *x += 1).or_insert(1);
        loop {
            let tmp_x = cur_x + dir_x;
            let tmp_y = cur_y + dir_y;
            if tmp_x < 0 || tmp_y < 0 || tmp_x >= obstacles.len() as i32 || tmp_y >= obstacles[0].len() as i32 {
                return false;
            }
            if obstacles[tmp_x as usize][tmp_y as usize] {
                match cur_dir {
                    Direction::Up => {
                        dir_x = 0;
                        dir_y = 1;
                        cur_dir = Direction::Right;
                    }
                    Direction::Right => {
                        dir_x = 1;
                        dir_y = 0;
                        cur_dir = Direction::Down;
                    }
                    Direction::Down => {
                        dir_x = 0;
                        dir_y = -1;
                        cur_dir = Direction::Left;
                    }
                    Direction::Left => {
                        dir_x = -1;
                        dir_y = 0;
                        cur_dir = Direction::Up;
                    }
                }
            } else {
                break;
            }
        }
        cur_x += dir_x;
        cur_y += dir_y;
    }
    true
}

fn solve(file: File) -> i128 {
    let mut ans: i128 = 0;
    let lines = std::io::BufReader::new(file).lines();

    let mut obstacles: Vec<Vec<bool>> = Vec::new();
    
    let mut cur_x: i32 = 0;
    let mut cur_y: i32 = 0;
    // Read until empty line
    for (x, line) in lines.enumerate() {
        obstacles.push(Vec::new());
        for (y, char) in line.unwrap().chars().enumerate() {
            obstacles[x].push(false);
            match char {
                '#' => {
                    obstacles[x][y] = true;
                }
                '^' => {
                    cur_x = x as i32;
                    cur_y = y as i32;
                }
                _ => {}
            }
        }
    }

    for i in 0..obstacles.len() {
        for j in 0..obstacles[0].len() {
            if obstacles[i][j] {
                continue;
            }
            // See if we're stuck if I put an obstacle here
            obstacles[i][j] = true;
            if simulate(&obstacles, cur_x, cur_y) {
                ans += 1;
            }
            obstacles[i][j] = false;
        }
    }

    ans.into()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    // log the time
    let start = Instant::now();
    let safe_cnt = solve(file);
    let duration = start.elapsed();
    println!("{}", safe_cnt);
    println!("Time taken: {:?}", duration);
}
