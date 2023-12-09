use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let file = File::open(args[1].as_str()).unwrap();
        let reader = io::BufReader::new(file);
        let nr_pattern = Regex::new(r"(\d+)").unwrap();

        // Count the number of non-empty rows using functional style
        let non_empty_row_count: usize = reader
            .lines()
            .filter(|line| {
                let line = line.as_ref().unwrap();
                !line.trim().is_empty()
            })
            .count();

        let file = File::open(args[1].as_str()).unwrap();
        let reader = io::BufReader::new(file);
        let mut nr_cards = vec![1; non_empty_row_count];

        let mut sum_part1 = 0;

        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);

            let card_split: Vec<&str> = line.split(":").collect();

            if card_split.len() == 2 {
                if let Some(id) = nr_pattern.captures(card_split[0]) {
                    let card_id = id.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    println!("Card: {}", card_id);

                    let mut winning_guesses: Vec<i32> = vec![];
                    let parts: Vec<&str> = card_split[1].split("|").collect();

                    if parts.len() == 2 {
                        let winners: Vec<&str> = parts[0].split(" ").collect();
                        let guesses: Vec<&str> = parts[1].split(" ").collect();
                        let mut i: usize = 0;
                        for win in &winners {
                            if let Some(digit) = nr_pattern.captures(win) {
                                let digit = digit.get(1).unwrap().as_str().parse::<i32>().unwrap();
                                for guess in &guesses {
                                    if guess == win {
                                        println!("Matching guess: {}", guess);
                                        winning_guesses.push(digit);
                                        nr_cards[card_id + i] += nr_cards[(card_id - 1) as usize];
                                        i += 1;
                                    }
                                }
                            }
                        }
                    }
                    if winning_guesses.len() > 0 {
                        let points =
                            2_i32.pow(((winning_guesses.len() - 1) as i32).try_into().unwrap());
                        println!("Card {} is worth {} points", card_id, points);
                        sum_part1 += points;
                    }
                }
            }
        }
        println!("Result of part 1: {}", sum_part1);
        println!("Result of part 2: {}", nr_cards.iter().sum::<i32>());
    }
}
