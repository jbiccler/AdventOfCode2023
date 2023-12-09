use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn map_card(c: &char, part1: bool) -> i32 {
    return match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if part1 {
                11
            } else {
                1
            }
        }
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    };
}

fn score_hand(hand: &str) -> i32 {
    // get unique chars
    let mut uniques: Vec<char> = hand.chars().collect();
    uniques.sort();
    uniques.dedup();

    let mut n_pairs = 0;
    let mut n_trips = 0;

    for c in uniques {
        let count = hand.chars().filter(|&char| char == c).count() as i32;
        if count >= 4 {
            // four or five of a kind
            return count + 1;
        } else if count == 3 {
            n_trips += 1;
        } else if count == 2 {
            n_pairs += 1;
        }
    }
    if n_trips == 1 && n_pairs == 1 {
        return 4;
    } else if n_trips == 1 {
        return 3;
    } else if n_pairs == 2 {
        return 2;
    } else {
        return n_pairs;
    }
}

fn score_hand_part2(hand: &str) -> i32 {
    // get unique chars
    let mut uniques: Vec<char> = hand.chars().collect();
    uniques.sort();
    uniques.dedup();

    let mut n_pairs = 0;
    let mut n_trips = 0;
    let n_jokers = hand.chars().filter(|&char| char == 'J').count() as i32;

    for c in uniques {
        let count = hand.chars().filter(|&char| char == c).count() as i32;
        if count == 5 {
            return count + 1;
        } else if count == 4 {
            // four or five of a kind
            if n_jokers == 1 || n_jokers == 4 {
                return count + 2;
            }
            return count + 1;
        } else if count == 3 {
            n_trips += 1;
        } else if count == 2 {
            n_pairs += 1;
        }
    }
    if n_trips == 1 && n_pairs == 1 {
        if n_jokers == 3 || n_jokers == 2 {
            return 6;
        }
        return 4;
    } else if n_trips == 1 {
        if n_jokers == 3 || n_jokers == 1 {
            return 5;
        } else if n_jokers == 2 {
            return 6;
        }
        return 3;
    } else if n_pairs == 2 {
        if n_jokers == 2 {
            return 5;
        } else if n_jokers == 1 {
            return 4;
        }
        return 2;
    } else if n_pairs == 1 {
        if n_jokers == 2 || n_jokers == 1 {
            return 3;
        }
        return 1;
    } else {
        if n_jokers == 1 {
            return 1;
        }
        return 0;
    }
}

fn parse_card(line: &str, part1: bool) -> (i32, i32, i32, i32, i32, i32, i32) {
    let spl: Vec<&str> = line.trim().split(" ").collect();
    if spl.len() == 2 {
        // let mut score: i32 = 0;
        let score = if part1 {
            score_hand(spl[0])
        } else {
            score_hand_part2(spl[0])
        };
        let mapped: Vec<i32> = spl[0].chars().map(|c| map_card(&c, part1)).collect();

        return (
            score,
            mapped[0],
            mapped[1],
            mapped[2],
            mapped[3],
            mapped[4],
            spl[1].parse::<i32>().unwrap(),
        );
    }
    return (0, 0, 0, 0, 0, 0, 0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let file = File::open(args[1].as_str()).unwrap();
        let reader = io::BufReader::new(file);
        let mut cards: Vec<(i32, i32, i32, i32, i32, i32, i32)> = vec![];
        let mut cards_part2: Vec<(i32, i32, i32, i32, i32, i32, i32)> = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            cards.push(parse_card(&line, true));
            cards_part2.push(parse_card(&line, false));
        }
        // last card is the best card
        cards.sort();
        // dbg!(&cards);
        let sum_part1 = cards
            .iter()
            .enumerate()
            .fold(0, |acc, (i, c)| acc + ((i + 1) as i32) * c.6);
        println!("Solution to part 1: {}", sum_part1);

        // last card is the best card
        cards_part2.sort();
        // dbg!(&cards);
        let sum_part2 = cards_part2
            .iter()
            .enumerate()
            .fold(0, |acc, (i, c)| acc + ((i + 1) as i32) * c.6);
        println!("Solution to part 2: {}", sum_part2);
    }
}
