use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

fn solve(lst_1: &mut Vec<i32>, lst_2: &mut Vec<i32>) -> i32 {
    lst_1.sort();
    lst_2.sort();

    let mut dist_sum = 0;
    for i in 0..lst_1.len() {
        dist_sum += (lst_1[i] - lst_2[i]).abs();
    }
    dist_sum
}

fn take_input(file: File) -> (Vec<i32>, Vec<i32>) {
    let mut lst_1 = Vec::new();
    let mut lst_2 = Vec::new();

    let mut reader = std::io::BufReader::new(file);
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        lst_1.push(nums[0]);
        lst_2.push(nums[1]);
        line.clear();
    }
    (lst_1, lst_2)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let (mut lst_1, mut lst_2) = take_input(file);
    println!("{}", solve(&mut lst_1, &mut lst_2));
}
