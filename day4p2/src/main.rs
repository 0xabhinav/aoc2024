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

#[derive(Debug)]
struct Choices {
    main_direction: Direction,
    alt_direction: Direction,
    alt_reverse: bool,
    dx: i32,
    dy: i32,
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

const CHARS_TO_MATCH: &str = "MAS";

fn is_match(vec_str: &Vec<String>, i: i32, j: i32, direction: &Direction, reverse: bool) -> bool {
    if i < 0 || j < 0 {
        return false;
    }
    let mut x_idx = i as usize;
    let mut y_idx = j as usize;
    let Some(line) = vec_str.get(x_idx) else {
        return false;
    };
    let Some(mut next_char) = line.chars().nth(y_idx) else {
        return false;
    };

    for (idx, c) in CHARS_TO_MATCH.chars().enumerate() {
        if next_char != c {
            return false;
        }
        if idx == 2 {
            return true;
        }
        if let Some(next_char_obj) = get_next_char(vec_str, x_idx, y_idx, direction, reverse) {
            x_idx = next_char_obj.x_idx;
            y_idx = next_char_obj.y_idx;
            next_char = next_char_obj.char;
        } else {
            return false;
        }
    }
    return false;
}

fn solve_part(vec_str: &Vec<String>, choice: &Choices, reverse: bool) -> u128 {
    // println!("choice: {:?}, reverse: {}", choice, reverse);
    // println!("direction: {:?}, reverse: {}", direction, reverse);
    let mut ans = 0;
    let dx = match reverse {
        true => -choice.dx,
        false => choice.dx,
    };
    let dy = match reverse {
        true => -choice.dy,
        false => choice.dy,
    };
    let alt_reverse = reverse ^ choice.alt_reverse;
    for i in 0..vec_str.len() {
        for j in 0..vec_str[0].len() {
            let main_match = is_match(vec_str, i as i32, j as i32, &choice.main_direction, reverse);
            let alt_match = is_match(vec_str, i as i32 + dx, j as i32 + dy, &choice.alt_direction, alt_reverse);
            if main_match && alt_match {
                // println!("i: {}, j: {}", i, j);
                ans += 1;
            }
        }

    }
    ans
}

fn solve(file: File) -> u128 {
    let mut ans = 0;
    let choices = vec![
        Choices {
            main_direction: Direction::Diagonal,
            alt_direction: Direction::InvertedDiagonal,
            alt_reverse: false,
            dx: 0,
            dy: 2,
        },
        Choices {
            main_direction: Direction::Diagonal,
            alt_direction: Direction::InvertedDiagonal,
            alt_reverse: true,
            dx: 2,
            dy: 0,
        },
    ];
    let lines = std::io::BufReader::new(file).lines();
    let lines_vec: Vec<String> = lines.map(|l| l.unwrap()).collect();
    for choice in choices {
        ans += solve_part(&lines_vec, &choice, false);
        ans += solve_part(&lines_vec, &choice, true);
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
