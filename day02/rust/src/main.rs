use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let file = File::open(args[1].as_str()).unwrap();
        let reader = io::BufReader::new(file);

        let game_pattern = Regex::new(r"Game (\d+)").unwrap();
        let rgb_pattern = Regex::new(r"\s(\d+)\s(blue|red|green)").unwrap();
        let mut sum_part1 = 0;
        let mut sum_part2 = 0;

        for line in reader.lines() {
            let line = line.unwrap();

            let parts: Vec<&str> = line.split(":").collect();

            if parts.len() == 2 {
                if let Some(game) = game_pattern.captures(parts[0]) {
                    let game_id = game.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let split_rounds: Vec<&str> = parts[1].split(";").collect();
                    let mut sum = vec![0, 0, 0];
                    for round in split_rounds {
                        // rgb
                        for pair in round.split(",").collect::<Vec<&str>>() {
                            if let Some(p) = rgb_pattern.captures(pair) {
                                if let Some(colour) = p.get(2) {
                                    match colour.as_str() {
                                        "red" => {
                                            sum[0] = std::cmp::max(
                                                sum[0],
                                                p.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                                            );
                                        }
                                        "green" => {
                                            sum[1] = std::cmp::max(
                                                sum[1],
                                                p.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                                            );
                                        }
                                        "blue" => {
                                            sum[2] = std::cmp::max(
                                                sum[2],
                                                p.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                                            );
                                        }
                                        _ => println!("No hits: {}", game_id),
                                    }
                                }
                            }
                        }
                    }
                    if sum[0] <= 12 && sum[1] <= 13 && sum[2] <= 14 {
                        sum_part1 += game_id;
                    }
                    let part2 = sum.iter().fold(1, |acc, x| acc * x);
                    sum_part2 += part2;
                }
            }
        }
        println!("Result part 1: {}", sum_part1);
        println!("Result part 2: {}", sum_part2);
    }
}
