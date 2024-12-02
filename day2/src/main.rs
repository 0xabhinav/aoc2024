use std::fs::File;
use std::io::prelude::*;

fn is_safe<I: Iterator<Item = i32>>(mut iterable: I) -> bool {
    let mut last_val = match iterable.next() {
        Some(val) => val,
        None => return true,
    };

    let mut is_decreasing = false;
    let mut is_increasing = false;
    for curr in iterable {   
        let diff = curr - last_val;
        if diff > 0 {
            is_increasing = true;
        } else {
            is_decreasing = true;
        }
        if is_increasing && is_decreasing {
            return false;
        }
        let diff = diff.abs();
        if diff > 3 || diff == 0 {
            return false;
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
            let iterable = l.split_whitespace()
                .map(|s| s.parse().unwrap());

            if is_safe(iterable) {
                Some(true)
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
