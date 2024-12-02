use std::fs::File;
use std::io::prelude::*;

fn is_safe<I: Iterator<Item = i32>>(mut iterable: I) -> bool {
    let mut last_val = match iterable.next() {
        Some(val) => val,
        None => return true,
    };

    let mut is_decreasing = false;
    let mut is_increasing = false;
    let mut found_invalid = false;
    for curr in iterable {   
        let diff = curr - last_val;
        let mut curr_increasing = false;
        if diff > 0 {
            is_increasing = true;
            curr_increasing = true;
        } else {
            is_decreasing = true;
        }
        if is_increasing && is_decreasing {
            if found_invalid {
                return false;
            }
            // Undo the last change
            if curr_increasing {
                is_increasing = false;
            } else {
                is_decreasing = false;
            }
            found_invalid = true;
            continue;
        }
        let diff = diff.abs();
        if diff > 3 || diff == 0 {
            if found_invalid {
                return false;
            }
            found_invalid = true;
            continue;
        }
        last_val = curr;
    }
    true
}

fn solve(file: File) -> usize {
    let safe_cnt = std::io::BufReader::new(file)
        .lines()
        .filter_map(|l| {
            let l = l.unwrap();
            let vec = l.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>();

            if is_safe(vec.iter().copied()) || is_safe(vec.iter().rev().copied()) {
                Some(())
            } else {
                None
            }
        })
        .count();
    
    safe_cnt
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let safe_cnt = solve(file);
    println!("{}", safe_cnt);
}
