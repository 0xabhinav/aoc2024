use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use std::collections::HashMap;

fn solve(file: File) -> i128 {
    let lines = std::io::BufReader::new(file).lines();

    let mut obstacles: Vec<Vec<bool>> = Vec::new();
    let mut visited_cnt: HashMap<(i32, i32), i32> = HashMap::new();
    let mut cur_x: i32 = 0;
    let mut cur_y: i32 = 0;
    let mut dir_x: i32 = 0;
    let mut dir_y: i32 = 1;
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
                    dir_x = -1;
                    dir_y = 0;
                }
                _ => {}
            }
        }
    }


    while *visited_cnt.get(&(cur_x, cur_y)).unwrap_or(&0) <= 4 {
        visited_cnt.entry((cur_x, cur_y)).and_modify(|x| *x += 1).or_insert(1);
        let mut done = false;
        loop {
            let tmp_x = cur_x + dir_x;
            let tmp_y = cur_y + dir_y;
            if tmp_x < 0 || tmp_y < 0 || tmp_x >= obstacles.len() as i32 || tmp_y >= obstacles[0].len() as i32 {
                done = true;
                break;
            }
            if obstacles[tmp_x as usize][tmp_y as usize] {
                match (dir_x, dir_y) {
                    (-1, 0) => {
                        dir_x = 0;
                        dir_y = 1;
                    }
                    (0, 1) => {
                        dir_x = 1;
                        dir_y = 0;
                    }
                    (1, 0) => {
                        dir_x = 0;
                        dir_y = -1;
                    }
                    (0, -1) => {
                        dir_x = -1;
                        dir_y = 0;
                    }
                    _ => {}
                }
            } else {
                break;
            }
        }
        if done {
            break;
        }
        cur_x += dir_x;
        cur_y += dir_y;
    }
    let ans = visited_cnt.len() as i128;
    // for i in 0..obstacles.len() {
    //     for j in 0..obstacles[0].len() {
    //         if *visited_cnt.get(&(i as i32, j as i32)).unwrap_or(&0) > 0 {
    //         print!("X");
    //         } else if obstacles[i][j] {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }

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
