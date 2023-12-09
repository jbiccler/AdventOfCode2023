use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn parse_input(file_path: &str) -> (Vec<i64>, Vec<i64>) {
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);
    let (mut times, mut distances) = (vec![], vec![]);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("Time") {
            times = line
                .split("Time:")
                .nth(1)
                .map(|s| s.trim().split_whitespace())
                .unwrap()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
        }
        if line.contains("Distance") {
            distances = line
                .split("Distance:")
                .nth(1)
                .map(|s| s.trim().split_whitespace())
                .unwrap()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
        }
    }
    return (times, distances);
}

fn distance_covered(t_race: i64, t_holding: i64) -> i64 {
    if (t_holding >= t_race) | (t_holding <= 0) {
        return 0;
    } else {
        let speed = t_holding;
        let t_left = t_race - t_holding;
        return t_left * speed;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let (times, distances) = parse_input(args[1].as_str());
        let mut score1 = 1;
        if times.len() == distances.len() {
            for i in 0..times.len() {
                let mut n_won = 0;
                let t = times[i];
                for h in 1..t {
                    let dist = distance_covered(t, h);
                    let won = dist > distances[i];
                    if won {
                        n_won += 1;
                    }
                }
                score1 *= n_won;
            }
            println!("Solution to part 1: {}", score1);
        }
        // part2
        let time: i64 = times
            .iter()
            .map(|&num| num.to_string())
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        let distance: i64 = distances
            .iter()
            .map(|&num| num.to_string())
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        let mut n_won2 = 0;
        for h in 1..time {
            let dist = distance_covered(time, h);
            let won = dist > distance;
            if won {
                n_won2 += 1;
            } else if n_won2 > 0 {
                break;
            }
        }
        println!("Solution to part 2: {}", n_won2);
    }
}
