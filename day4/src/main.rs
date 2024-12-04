use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

#[derive(Debug)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
    InvertedDiagonal,
}

struct NextChar {
    x_idx: usize,
    y_idx: usize,
    char: char,
}

fn get_next_char<'a>(vec_str: &'a Vec<String>, x_idx: usize, y_idx: usize, direction: &Direction, reverse: bool) -> Option<NextChar> {
    let mut dx;
    let mut dy;
    match direction {
        Direction::Horizontal => {
            dx = 0;
            dy = 1;
        }
        Direction::Vertical => {
            dx = 1;
            dy = 0;
        }
        Direction::Diagonal => {
            dx = 1;
            dy = 1;
        }
        Direction::InvertedDiagonal => {
            dx = 1;
            dy = -1;
        }
    }
    if reverse {
        dx = -dx;
        dy = -dy;
    }
    let x_idx = x_idx as i32 + dx;
    let y_idx = y_idx as i32 + dy;
    if x_idx < 0 || y_idx < 0 || x_idx >= vec_str.len() as i32 || y_idx >= vec_str[0].len() as i32 {
        return None;
    }
    return Some(NextChar {
        x_idx: x_idx as usize,
        y_idx: y_idx as usize,
        char: vec_str[x_idx as usize].chars().nth(y_idx as usize).unwrap(),
    });
}

const CHARS_TO_MATCH: &str = "XMAS";

fn solve_part(vec_str: &Vec<String>, direction: &Direction, reverse: bool) -> u128 {
    // println!("direction: {:?}, reverse: {}", direction, reverse);
    let mut ans = 0;
    for i in 0..vec_str.len() {
        for j in 0..vec_str[0].len() {
            let mut x_idx = i;
            let mut y_idx = j;
            let mut next_char = vec_str[x_idx].chars().nth(y_idx).unwrap();
            for (idx, c) in CHARS_TO_MATCH.chars().enumerate() {
                if next_char != c {
                    break;
                }
                if idx == 3 {
                    // println!("{},{}", i, j);
                    ans += 1;
                }
                if let Some(next_char_obj) = get_next_char(vec_str, x_idx, y_idx, direction, reverse) {
                    x_idx = next_char_obj.x_idx;
                    y_idx = next_char_obj.y_idx;
                    next_char = next_char_obj.char;
                } else {
                    break;
                }
            }
            // Subset variation
            // if vec_str[i].chars().nth(j as usize).unwrap() != 'X' {
            //     continue;
            // }
            // let mut counts = [0; 4];
            // counts[0] = 1;
            // while let Some(next_char) = get_next_char(vec_str, x_idx, y_idx, direction, reverse) {
            //     x_idx = next_char.x_idx;
            //     y_idx = next_char.y_idx;

                
            //     for (idx, c) in CHARS_TO_MATCH.chars().skip(1).enumerate() {
            //         if next_char.char == c {
            //             counts[idx + 1] += counts[idx];
            //         }
            //     }
            // }
            // if counts[3] > 0 {
            //     println!("{},{}: {}", i, j, counts[3]);
            // }
            // ans += counts[3];
        }

    }
    ans
}

fn solve(file: File) -> u128 {
    let mut ans = 0;
    let choices = vec![
        Direction::Horizontal,
        Direction::Vertical,
        Direction::Diagonal,
        Direction::InvertedDiagonal,
    ];
    let lines = std::io::BufReader::new(file).lines();
    let lines_vec: Vec<String> = lines.map(|l| l.unwrap()).collect();
    for direction in choices {
        ans += solve_part(&lines_vec, &direction, false);
        ans += solve_part(&lines_vec, &direction, true);
    }
    ans
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
