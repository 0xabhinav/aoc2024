use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use std::collections::{HashMap, HashSet};

fn solve_one(idxs: &Vec<(i64, i64)>, max_x: i64, max_y: i64, antinode_locs: &mut HashSet<(i64, i64)>) {
    for (x, y) in idxs.iter() {
        for (x2, y2) in idxs.iter() {
            let dx = x - x2;
            let dy = y - y2;
            if dx == 0 && dy == 0 {
                continue;
            }
            let mut x3 = *x;
            let mut y3 = *y;
            loop {
                x3 = x3 + dx;
                y3 = y3 + dy;
                if x3 < 0 || x3 > max_x || y3 < 0 || y3 > max_y {
                    break;
                }
                antinode_locs.insert((x3, y3));
            }
        }
    }
}

fn solve(file: File) -> i128 {
    let ans;
    let lines = std::io::BufReader::new(file).lines();

    let mut antenna_idxs = HashMap::<char, Vec<(i64, i64)>>::new();
    let mut antinode_locs = HashSet::<(i64, i64)>::new();
    let mut max_x = 0;
    let mut max_y = 0;
    // Read until empty line
    for (i, line) in lines.enumerate() {
        // num: num1 num2 num3 num4 ...
        let line = line.unwrap();
        for (j, ch) in line.chars().enumerate() {
            if (ch >= 'A' && ch <= 'Z') || (ch >= 'a' && ch <= 'z') || (ch >= '0' && ch <= '9') {
                antenna_idxs.entry(ch).or_insert(Vec::new()).push((i as i64, j as i64));
                antinode_locs.insert((i as i64, j as i64));
            }
            max_y = max_y.max(j as i64);
        }
        max_x = max_x.max(i as i64);
    }

    for (_, locs) in antenna_idxs.iter() {
        solve_one(&locs, max_x, max_y, &mut antinode_locs);
    }

    // for i in 0..=max_x {
    //     for j in 0..=max_y {
    //         if antinode_locs.contains(&(i, j)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    ans = antinode_locs.len() as i128;
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
