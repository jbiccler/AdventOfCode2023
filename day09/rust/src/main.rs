use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn interpolate(v: &Vec<i64>, start: bool) -> Option<(i64, i64)> {
    if v.len() <= 1 {
        return None;
    }
    let mut res = vec![0; v.len() - 1];
    let mut n = 0;
    for i in 0..(v.len() - 1) {
        res[i] = v[i + 1] - v[i];
        if res[i] == 0 {
            n += 1;
        }
    }
    if n == res.len() {
        return Some((0, 0));
    } else {
        let (next_last, next_first) = interpolate(&res, false).unwrap();
        if start {
            let last = next_last + res.iter().last().unwrap();
            let first = res[0] - next_first;
            return Some((v.iter().last().unwrap() + last, v[0] - first));
        } else {
            return Some((next_last + res.iter().last().unwrap(), res[0] - next_first));
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let file = File::open(args[1].as_str()).unwrap();
        let reader = io::BufReader::new(file);
        let mut sum1 = 0;
        let mut sum2 = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let spl: Vec<i64> = line
                .split_whitespace()
                .map(|d| d.parse::<i64>().unwrap())
                .collect();
            // dbg!(&spl);
            let (last, first) = interpolate(&spl, true).unwrap();
            sum1 += last;
            sum2 += first;
            // println!("interpolated: {}", inter);
        }
        println!("Solution to part 1: {}", sum1);
        println!("Solution to part 2: {}", sum2);
    }
}
