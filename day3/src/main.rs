use std::fs::File;
use std::io::prelude::*;
use regex::Regex;


fn solve(file: File) -> u128 {
    // mul(num, num) regex
    let pattern = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let mut ans: u128 = 0;
    // Iterate over the lines
    for line in lines {
        let Ok(line) = line else {
            continue
        };
        // Keep capturing from the left and trimming the string
        let mut curr_line = line;
        while let Some(captures) = pattern.captures(&curr_line) {
            let num1 = captures.get(1).unwrap().as_str().parse::<u128>().unwrap();
            let num2 = captures.get(2).unwrap().as_str().parse::<u128>().unwrap();
            ans += num1 * num2;
            let end = captures.get(0).unwrap().end();
            curr_line = curr_line[end..].to_string();
        }
    }
    ans
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let safe_cnt = solve(file);
    println!("{}", safe_cnt);
}
