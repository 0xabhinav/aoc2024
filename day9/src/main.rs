use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn solve(file: File) -> i128 {
    let mut ans = 0;
    let mut lines = std::io::BufReader::new(file).lines();
    let line = lines.next().unwrap().unwrap();
    let mut sizes = vec![];
    let mut is_free_space = false;
    let mut free_spaces = vec![];
    for ch in line.chars() {
        if ch >= '0' && ch <= '9' {
            let digit = ch.to_digit(10).unwrap() as i64;
            if is_free_space {
                free_spaces.push(digit);
            } else {
                sizes.push(digit);
            }
            is_free_space = !is_free_space;
        }
    }
    free_spaces.reverse();

    is_free_space = false;
    let mut idx = 0;
    let mut sizes_idx = 0;
    let mut sizes_back_idx = sizes.len() - 1;
    loop {
        if !is_free_space && sizes.len() > 0 {
            for _ in 0..sizes[0] {
                // print!("{}", sizes_idx);
                ans += (idx * (sizes_idx)) as i128;
                idx += 1;
            }
            sizes_idx += 1;
            is_free_space = !is_free_space;
            // pop front
            sizes.remove(0);
        } else if is_free_space && free_spaces.len() > 0 {
            if sizes.len() == 0 {
                break;
            }
            let last_size_idx = sizes.len() - 1;
            let last_free_idx = free_spaces.len() - 1 as usize;
            if sizes[last_size_idx] >= free_spaces[last_free_idx] {
                for _ in 0..free_spaces[last_free_idx] {
                    // print!("{}", sizes_back_idx);
                    ans += (idx * (sizes_back_idx)) as i128;
                    idx += 1;
                }
                sizes[last_size_idx] -= free_spaces[last_free_idx];
                free_spaces.pop();
                if sizes[last_size_idx] == 0 {
                    sizes.pop();
                    sizes_back_idx -= 1;
                }
                is_free_space = !is_free_space;
            } else {
                for _ in 0..sizes[last_size_idx] {
                    // print!("{}", sizes_back_idx);
                    ans += (idx * (sizes_back_idx)) as i128;
                    idx += 1;
                }
                free_spaces[last_free_idx] -= sizes[last_size_idx];
                sizes.pop();
                sizes_back_idx -= 1;
            }
        } else {
            break;
        }
    }
    // println!("");

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
