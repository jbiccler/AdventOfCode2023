use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn map_rl(c: &char) -> Option<usize> {
    match c {
        'R' => return Some(1),
        'L' => return Some(0),
        _ => return None,
    }
}

fn parse_rl(line: &str) -> Vec<usize> {
    return line.chars().map(|c| map_rl(&c).unwrap()).collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let file = File::open(args[1].as_str()).unwrap();
        let reader = io::BufReader::new(file);
        let mut lines = reader.lines();
        let dirs = parse_rl(&lines.next().unwrap().unwrap());

        let mut map: HashMap<String, [String; 2]> = HashMap::new();
        let pattern = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

        for line in lines {
            let line = line.unwrap().clone();
            if line.len() > 0 {
                if let Some(capture) = pattern.captures(&line) {
                    let fr = capture.get(1).unwrap().as_str().to_string();
                    let to1 = capture.get(2).unwrap().as_str().to_string();
                    let to2 = capture.get(3).unwrap().as_str().to_string();

                    map.insert(fr, [to1, to2]);
                }
            }
        }
        let mut current = "AAA";
        let mut i: usize = 0;
        let mut sum1 = 0;
        while current != "ZZZ" {
            let dir: usize = dirs[i % dirs.len()];
            current = map[current].get(dir).unwrap();
            sum1 += 1;
            i += 1;
        }
        println!("Solution to part 1: {}", sum1);

        //part2
        let currents: Vec<&str> = map
            .iter()
            .filter(|(key, _)| key.chars().last().unwrap() == 'A')
            .map(|(key, _)| key.as_str())
            .collect();

        let mut counts: Vec<i64> = vec![0; currents.len()];
        for c in 0..currents.len() {
            i = 0;
            current = currents[c];
            while current.chars().last().unwrap() != 'Z' {
                let dir: usize = dirs[i % dirs.len()];
                current = map[current].get(dir).unwrap();
                counts[c] += 1;
                i += 1;
            }
        }
        println!(
            "Solution to part 2: {}",
            counts.iter().fold(counts[0], |acc, &x| lcm(acc, x))
        );
    }
}
