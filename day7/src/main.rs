use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn solve_one(expected: i128, nums: Vec<i128>) -> i128 {
    let mut possible_vals = vec![expected];
    for num in nums.iter().rev().copied() {
        let mut next_possible_vals = vec![];
        next_possible_vals.reserve(possible_vals.len() * 2);
        // Do operations in reverse order and see if we can get to 0
        for val in possible_vals {
            if val - num >= 0 {
                next_possible_vals.push(val - num);
            }
            if val % num == 0 {
                next_possible_vals.push(val / num);
            }
        }
        possible_vals = next_possible_vals;
    }
    possible_vals.iter().filter(|&&x| x == 0).count() as i128
}

fn solve(file: File) -> i128 {
    let mut ans = 0;
    let lines = std::io::BufReader::new(file).lines();

    // Read until empty line
    for line in lines {
        // num: num1 num2 num3 num4 ...
        let line = line.unwrap();
        let split = line.split(":").collect::<Vec<&str>>();
        let expected = split[0].parse::<i128>().unwrap();
        let nums = split[1].trim().split(" ").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<i128>>();

        if solve_one(expected, nums) > 0 {
            ans += expected;
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
