use std::{collections::HashSet, error::Error, fs::read_to_string, time::Instant};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let raw = read_to_string("inputs/dec04.txt")?;
    let now = Instant::now();
    let result = handle_puzzle1(&raw);

    println!("Test 1: {result}, {}µs", now.elapsed().as_millis());

    let now = Instant::now();
    let result = handle_puzzle2(&raw);
    println!("Test 2: {result}, {}µs", now.elapsed().as_millis());
    Ok(())
}

fn handle_puzzle1(raw: &str) -> i64 {
    raw.lines()
        .map(|line| {
            let (_, nums) = line.split_once(':').unwrap();
            let (left, right) = nums.split_once('|').unwrap();

            let left = left
                .split_whitespace()
                .map(|num| num.trim().parse::<i8>().unwrap())
                .collect::<HashSet<_>>();
            let right = right
                .split_whitespace()
                .map(|num| num.trim().parse().unwrap())
                .collect();
            let count_matches = left.intersection(&right).count();

            if count_matches == 0 {
                0
            } else {
                1 << (count_matches - 1)
            }
        })
        .sum()
}

fn handle_puzzle2(raw: &str) -> i64 {
    let values = raw
        .lines()
        .map(|line| {
            let (_, nums) = line.split_once(':').unwrap();
            let (left, right) = nums.split_once('|').unwrap();

            let left = left
                .split_whitespace()
                .map(|num| num.trim().parse::<i8>().unwrap())
                .collect::<HashSet<_>>();
            let right = right
                .split_whitespace()
                .map(|num| num.trim().parse().unwrap())
                .collect();
            left.intersection(&right).count()
        })
        .collect::<Vec<_>>();

    let mut cards = vec![1_i64; values.len()];
    for (i, jump) in values.iter().enumerate() {
        for j in 1..=*jump {
            cards[i + j] += cards[i];
        }
    }

    cards.iter().sum()
}

#[test]
fn test_puzzle1() {
    let test_input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    assert_eq!(handle_puzzle1(test_input), 13);
}

#[test]
fn test_puzzle2() {
    let test_input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    assert_eq!(handle_puzzle2(test_input), 30);
}
