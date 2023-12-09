use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn parse_ranges(line: &str) -> (i64, i64, i64) {
    let spl: Vec<i64> = line
        .split(" ")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();
    if spl.len() == 3 {
        let (dest, source, n) = (spl[0], spl[1], spl[2]);
        return (source, source + n - 1, dest - source);
    }
    return (0, 0, 0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let file = File::open(args[1].as_str()).unwrap();
        let reader = io::BufReader::new(file);
        let mut map: Vec<(i64, i64, i64)> = vec![];
        let mut result: Vec<Vec<(i64, i64, i64)>> = vec![];
        let mut seeds1: Vec<i64> = vec![];
        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if i == 0 {
                seeds1 = line
                    .split(": ")
                    .nth(1) // Take the part after the colon
                    .map(|s| s.trim().split_whitespace()) // Split the numbers
                    .unwrap() // Handle the case where there are no numbers after the colon
                    .map(|num_str| num_str.parse::<i64>().unwrap()) // Parse each number to usize
                    .collect();
                // dbg!(&seeds);
            } else if !line.contains("map") && line.trim().len() > 0 {
                // parse digits
                map.push(parse_ranges(&line));
            } else if line.trim().len() == 0 && i > 3 {
                result.push(map.clone());
                // reset map
                map.clear();
            }
        }
        // push the last one as well
        result.push(map);
        // dbg!(&result);
        let mut loc: Vec<i64> = vec![];
        for seed in &seeds1 {
            let mut current: i64 = *seed;
            for i in 0..result.len() {
                for v in &result[i] {
                    if current >= v.0 && current <= v.1 {
                        current = v.0 + (current - v.0) + v.2;
                        break;
                    }
                }
            }
            println!("seed: {} has location: {}", seed, current);
            loc.push(current);
        }
        println!("Solution to part 1: {}", loc.iter().min().unwrap());

        let mut current_min = std::i64::MAX;

        for s in 0..seeds1.len() {
            if s % 2 != 0 {
                continue;
            }
            let start_seed = seeds1[s];
            let n_seeds = seeds1[s + 1];

            for seed in start_seed..(start_seed + n_seeds) {
                let mut current = seed;
                for i in 0..result.len() {
                    for v in &result[i] {
                        if current >= v.0 && current <= v.1 {
                            current = v.0 + (current - v.0) + v.2;
                            break;
                        }
                    }
                }
                if current < current_min {
                    current_min = current;
                }
            }
        }
        println!("Solution to part 2: {}", current_min);
    }
}
