use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn log10(val: u128) -> u32 {
    let mut ans = 0;
    let mut x = val;
    while x > 0 {
        ans += 1;
        x /= 10;
    }
    ans
}

fn solve_one(val: u128, iters: u128, memo: &mut HashMap<(u128, u128), u128>) -> u128 {
    if iters == 0 {
        return 1;
    }
    if let Some(ans) = memo.get(&(val, iters)) {
        return *ans;
    }
    let num_digits = log10(val);
    let result = match num_digits {
        0 => {
            solve_one(1, iters - 1, memo)
        }
        n if n > 0 && n % 2 == 0 => {
            let left_half = val / 10_u128.pow(num_digits / 2);
            let right_half = val % 10_u128.pow(num_digits / 2);
            solve_one(left_half, iters - 1, memo) + solve_one(right_half, iters - 1, memo)
        }
        _ => {
            solve_one(val * 2024, iters - 1, memo)
        }
    };
    memo.insert((val, iters), result);
    result
}

fn solve(file: File) -> u128 {
    let mut ans = 0;
    let mut lines = std::io::BufReader::new(file).lines();
    let line = lines.next().unwrap().unwrap();
    let parts = line.split_whitespace().map(|s| s.parse::<u128>().unwrap());
    let mut memo = HashMap::<(u128, u128), u128>::new();
    for element in parts {
        ans += solve_one(element, 25, &mut memo);
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
