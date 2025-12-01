use bstr::io::BufReadExt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

fn parse_direction(line: &[u8]) -> i32 {
    let (sign, digits) = match line.first() {
        Some(b'L') => (-1, &line[1..]),
        Some(b'R') => (1, &line[1..]),
        _ => panic!("Invalid direction"),
    };
    sign * digits.iter().fold(0i32, |acc, &b| acc * 10 + (b - b'0') as i32)
}

fn take_input_bstr(file: File) -> (i32, i32) {
    let mut idx: i32 = 50;
    let mut ans = 0;
    let mut ans_2 = 0;

    let mut reader = BufReader::new(file);
    reader.for_byte_line(|line| {
        let mut x = parse_direction(line);

        ans_2 += x.abs() / 100;
        x %= 100;

        if idx == 0 {
            idx = (100 + idx + x) % 100;
            return Ok(true);
        }

        idx += x;
        if idx < 0 {
            idx += 100;
            ans_2 += 1;
        } else if idx >= 100 {
            idx -= 100;
            if idx != 0 {
                ans_2 += 1;
            }
        }
        if idx == 0 {
            ans += 1;
            ans_2 += 1;
        }
        Ok(true)
    }).expect("Failed to read file");
    
    (ans, ans_2)
}

fn take_input(file: File) -> (i32, i32) {
    let mut idx: i32 = 50;
    let mut ans = 0;
    let mut ans_2 = 0;

    let mut reader = std::io::BufReader::new(file);
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        let mut closure = || {
            let mut byte_chars = line.trim_end().bytes();
            let mut x = match byte_chars.next() {
                Some(b'L') => {
                    // Parse the remaining bytes directly, without converting to utf8
                    let num = byte_chars
                        .fold(0i32, |acc, b| acc * 10 + (b - b'0') as i32);
                    -num
                },
                Some(b'R') => {
                    let num = byte_chars
                        .fold(0i32, |acc, b| acc * 10 + (b - b'0') as i32);
                    num
                },
                _ => panic!("Invalid direction"),
            };
            ans_2 += x.abs() / 100;
            x %= 100;

            if idx == 0 {
                idx = (100 + idx + x) % 100;
                return;
            }

            idx += x;
            if idx < 0 {
                idx += 100;
                ans_2 += 1;
            } else if idx >= 100 {
                idx -= 100;
                if idx != 0 {
                    ans_2 += 1;
                }
            }
            if idx == 0 {
                ans += 1;
                ans_2 += 1;
            }
        };
        closure();
        line.clear();
    }
    (ans, ans_2)
}

fn take_input_optimized(file: File) -> (i32, i32) {
    let mut idx: i32 = 50;
    let mut ans = 0;
    let mut ans_2 = 0;

    let mut reader = std::io::BufReader::new(file);
    let mut line = Vec::<u8>::new();
    while reader.read_until(b'\n', &mut line).unwrap() > 0 {
        let line_no_newline = if line.last() == Some(&b'\n') {
            &line[..line.len() - 1]
        } else {
            &line
        };
        let mut line_it = line_no_newline.iter();
        let mut x = match line_it.next() {
            Some(b'L') => {
                // Parse the remaining bytes directly, without converting to utf8
                let num = line_it
                    .fold(0i32, |acc, b| acc * 10 + (b - b'0') as i32);
                -num
            },
            Some(b'R') => {
                let num = line_it
                    .fold(0i32, |acc, b| acc * 10 + (b - b'0') as i32);
                num
            },
            _ => panic!("Invalid direction"),
        };

        ans_2 += x.abs() / 100;
        x %= 100;

        if idx == 0 {
            idx = (100 + idx + x) % 100;
            line.clear();
            continue;
        }

        idx += x;
        if idx < 0 {
            idx += 100;
            ans_2 += 1;
        } else if idx >= 100 {
            idx -= 100;
            if idx != 0 {
                ans_2 += 1;
            }
        }
        if idx == 0 {
            ans += 1;
            ans_2 += 1;
        }
        line.clear();
    }
    (ans, ans_2)
}

fn main() {
    {
        let file = File::open("input.txt").unwrap();
        let time = Instant::now();
        let (ans, ans_2) = take_input(file);
        let duration = time.elapsed();
        println!("{} {}", ans, ans_2);
        println!("Time taken: {:?}", duration);
    }

    {
        let file = File::open("input.txt").unwrap();
        let time = Instant::now();
        let (ans, ans_2) = take_input_optimized(file);
        let duration = time.elapsed();
        println!("{} {}", ans, ans_2);
        println!("Time taken: {:?}", duration);
    }

    {
        let file = File::open("input.txt").unwrap();
        let time = Instant::now();
        let (ans, ans_2) = take_input_bstr(file);
        let duration = time.elapsed();
        println!("{} {}", ans, ans_2);
        println!("Time taken: {:?}", duration);
    }
}
