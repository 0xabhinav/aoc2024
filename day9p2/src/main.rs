use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

enum ElementType {
    Size(usize, usize),
    FreeSpace(usize),
}

fn solve(file: File) -> i128 {
    let mut ans = 0;
    let mut lines = std::io::BufReader::new(file).lines();
    let line = lines.next().unwrap().unwrap();
    let mut elements = VecDeque::new();
    let mut is_free_space = false;
    let mut counter = 0;
    for ch in line.chars() {
        if ch >= '0' && ch <= '9' {
            let digit = ch.to_digit(10).unwrap() as usize;
            if is_free_space {
                elements.push_back(ElementType::FreeSpace(digit));
            } else {
                elements.push_back(ElementType::Size(counter, digit));
                counter += 1;
            }
            is_free_space = !is_free_space;
        }
    }

    let mut idx = 0;
    while elements.len() > 0 {
        match elements[0] {
            ElementType::Size(sizes_idx, size) => {
                for _ in 0..size {
                    // print!("{}", sizes_idx);
                    ans += (idx * (sizes_idx)) as i128;
                    idx += 1;
                }
                elements.pop_front();
            }
            ElementType::FreeSpace(mut free_size) => {
                for back_idx in (0..elements.len()).rev() {
                    let ElementType::Size(sizes_idx, size) = elements[back_idx] else {
                        continue;
                    };
                    if size <= free_size {
                        for _ in 0..size {
                            // print!("{}", sizes_idx);
                            ans += (idx * sizes_idx) as i128;
                            idx += 1;
                        }
                        free_size -= size;
                        // This idx is free now
                        elements[back_idx] = ElementType::FreeSpace(size);
                    }
                }
                // for _ in 0..free_size {
                //     print!(".");
                // }
                idx += free_size;
                elements.pop_front();
            }
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
