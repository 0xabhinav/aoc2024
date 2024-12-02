use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
use std::collections::HashMap;

fn solve(lst_1: &Vec<i32>, map_2: &HashMap<i32, u32>) -> i128 {

    let mut ans: i128 = 0;
    for i in 0..lst_1.len() {
        let count: i128 = *map_2.get(&lst_1[i]).unwrap_or(&(0 as u32)) as i128;
        ans += lst_1[i] as i128 * count;
    }
    ans
}

fn take_input(file: File) -> (Vec<i32>, HashMap<i32, u32>) {
    let mut lst_1 = Vec::new();
    let mut map_2 = HashMap::new();

    let mut reader = std::io::BufReader::new(file);
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        lst_1.push(nums[0]);
        *map_2.entry(nums[1]).or_insert(0) += 1;
        line.clear();
    }
    (lst_1, map_2)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let (lst_1, map_2) = take_input(file);
    println!("{}", solve(&lst_1, &map_2));
}
