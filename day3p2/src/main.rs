use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

const ENABLE_TEXT: &str = "do()";
const DISABLE_TEXT: &str = "don't()";
const EMPTY_STR: &str = "";

fn solve_part<'a>(in_str: &'a str, is_enabled: bool, pattern: &Regex) -> (u128, &'a str, bool) {
    if is_enabled {
        let next_disable_index = in_str.find(DISABLE_TEXT).unwrap_or(in_str.len() - DISABLE_TEXT.len()) + DISABLE_TEXT.len();
        // Find the pattern
        let Some(captures) = pattern.captures(&in_str) else {
            // If no pattern found, we're done
            return (0 as u128, EMPTY_STR, true);
        };
        // Check the start index of the pattern
        let start_index = captures.get(0).unwrap().start();
        if start_index < next_disable_index {
            let num1 = captures.get(1).unwrap().as_str().parse::<u128>().unwrap();
            let num2 = captures.get(2).unwrap().as_str().parse::<u128>().unwrap();
            let end = captures.get(0).unwrap().end();
            return (num1 * num2, &in_str[end..], true);
        } else {
            let in_str = &in_str[next_disable_index..];
            return (0 as u128, in_str, false);
        }
    } else {
        let Some(next_enable_index) = in_str.find(ENABLE_TEXT) else {
            return (0 as u128, EMPTY_STR, false);
        };
        // Trim the string to next_enable_index
        let in_str = &in_str[next_enable_index..];
        return (0 as u128, in_str, true);
    }
}

fn solve(file: File) -> u128 {
    // mul(num, num) regex
    let pattern = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();

    let lines = std::io::BufReader::new(file).lines();
    let mut ans: u128 = 0;
    let mut mul_enabled = true;
    // Iterate over the lines
    for line in lines {
        let Ok(line) = line else {
            continue
        };
        let mut curr_line: &str = &line;
        loop {
            let part_ans;
            (part_ans, curr_line, mul_enabled) = solve_part(curr_line, mul_enabled, &pattern);
            ans += part_ans;
            if curr_line.len() == 0 {
                break;
            }
        }
    }
    ans
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let safe_cnt = solve(file);
    println!("{}", safe_cnt);
}
