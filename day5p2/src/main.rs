use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use std::collections::HashSet;

fn is_valid(nums: &[u64], pairs: &[(u64, u64)]) -> bool {
    let mut seen_set: HashSet<u64> = HashSet::new();
    for num in nums.iter() {
        for (a, b) in pairs.iter() {
            if *a == *num && seen_set.contains(b) {
                return false;
            }
        }
        seen_set.insert(*num);
    }
    true
}

fn solve(file: File) -> u128 {
    let mut ans = 0;
    let mut lines = std::io::BufReader::new(file).lines();
    // Read until empty line
    let mut pairs: Vec<(u64, u64)> = Vec::new();
    while let Some(line) = lines.by_ref().next() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        // num|num
        let nums: Vec<u64> = line.split('|').map(|s| s.parse().unwrap()).collect();
        pairs.push((nums[0], nums[1]));
    }

    while let Some(line) = lines.by_ref().next() {
        // parse comma separated numbers
        let mut nums: Vec<u64> = line.unwrap().split(',').map(|s| s.parse().unwrap()).collect();
        if is_valid(&nums, &pairs) {
            continue;
        }
        // sort nums with criteria
        nums.sort_by(|a, b| {
            for (pair_first, pair_second) in pairs.iter() {
                if *b == *pair_first && *a == *pair_second {
                    return std::cmp::Ordering::Less;
                }
            }
            std::cmp::Ordering::Equal
        });
        let mid_idx = (nums.len() - 1) / 2;
        ans += nums[mid_idx];
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
