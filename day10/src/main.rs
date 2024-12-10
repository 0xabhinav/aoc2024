use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use std::collections::{HashMap, HashSet};

fn solve(file: File) -> i128 {
    let mut ans = 0;
    let lines = std::io::BufReader::new(file).lines();
    let mut heights = vec![];
    for line in lines {
        let line = line.unwrap();
        let mut height = vec![];
        for ch in line.chars() {
            height.push(ch.to_digit(10).unwrap() as usize);
        }
        heights.push(height);
    }
    let max_height = heights.len();
    let max_width = heights[0].len();
    let mut scores = HashMap::<(usize, usize), HashSet<(usize, usize)>>::new();
    // First mark all 9 with score 1
    for i in 0..max_height {
        for j in 0..max_width {
            if heights[i][j] == 9 {
                let mut set = HashSet::new();
                set.insert((i, j));
                scores.insert((i, j), set);
            }
        }
    }

    for digit in (0..9).rev() {
        for i in 0..max_height {
            for j in 0..max_width {
                if heights[i][j] == digit {
                    let mut set = HashSet::new();
                    // Combine the set of the 4 neighbors
                    if i > 0 && heights[i - 1][j] == digit + 1 {
                        set.extend(scores.get(&(i - 1, j)).unwrap());
                    }
                    if i < max_height - 1 && heights[i + 1][j] == digit + 1 {
                        set.extend(scores.get(&(i + 1, j)).unwrap());
                    }
                    if j > 0 && heights[i][j - 1] == digit + 1 {
                        set.extend(scores.get(&(i, j - 1)).unwrap());
                    }
                    if j < max_width - 1 && heights[i][j + 1] == digit + 1 {
                        set.extend(scores.get(&(i, j + 1)).unwrap());
                    }
                    scores.insert((i, j), set);
                }
            }
        }
    }

    for i in 0..max_height {
        for j in 0..max_width {
            if heights[i][j] == 0  {
                ans += scores.get(&(i, j)).unwrap().len() as i128;
            }
        }
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
