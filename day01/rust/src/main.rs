use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn string_to_i32(s: &str) -> Option<i32> {
    match s.to_lowercase().as_str() {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let file = File::open(&args[1]).unwrap();
        let reader = io::BufReader::new(file);
        let mut sum1: i32 = 0;
        let mut sum2: i32 = 0;
        let pattern1 =
            Regex::new(r"((one|two|three|four|five|six|seven|eight|nine)|(\d))(.*)").unwrap();
        let pattern2 =
            Regex::new(r"(.*)((one|two|three|four|five|six|seven|eight|nine)|(\d))").unwrap();

        for line in reader.lines() {
            let line = line.unwrap();
            let mut digits1 = "".to_string();
            // first part
            for char in line.chars() {
                if char.is_numeric() {
                    digits1 = digits1 + &char.to_string();
                    break;
                }
            }
            for char in line.chars().rev() {
                if char.is_numeric() {
                    digits1 = digits1 + &char.to_string();
                    break;
                }
            }
            sum1 += digits1.parse::<i32>().unwrap();

            // Second part
            let mut digits2 = "".to_string();
            if let Some(captures) = pattern1.captures(&line) {
                if let Some(group) = captures.get(2) {
                    digits2 += &string_to_i32(group.as_str()).unwrap().to_string();
                } else if let Some(group) = captures.get(3) {
                    digits2 += group.as_str();
                }
            }
            if let Some(captures) = pattern2.captures(&line) {
                if let Some(group) = captures.get(3) {
                    digits2 += &string_to_i32(group.as_str()).unwrap().to_string();
                } else if let Some(group) = captures.get(4) {
                    digits2 += group.as_str();
                }
            }
            sum2 += digits2.parse::<i32>().unwrap();
        }
        println!("Part 1: {}", sum1);
        println!("Part 2: {}", sum2);
    }
}
