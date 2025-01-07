#![feature(let_chains)]
use std::collections::HashMap;
use std::time::Instant;
use std::{error::Error, fs::read_to_string};

use regex::Regex;
fn main() -> Result<(), Box<dyn Error>> {
    let raw = read_to_string("inputs/dec08.txt")?;

    let now = Instant::now();
    let result = handle_puzzle1(&raw);

    println!("Test 1: {result}, {}µs", now.elapsed().as_millis());

    let now = Instant::now();
    let result = handle_puzzle2(&raw);
    println!("Test 2: {result}, {}µs", now.elapsed().as_millis());
    Ok(())
}

fn parse(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let (steps, edges) = input.split_once("\n\n").unwrap();
    let re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    let edges = edges.lines().fold(HashMap::new(), |mut acc, next| {
        let captures = re.captures(next).unwrap();
        let key = captures.get(1).unwrap().as_str();
        let left = captures.get(2).unwrap().as_str();
        let right = captures.get(3).unwrap().as_str();
        acc.insert(key, (left, right));
        acc
    });

    (steps.chars().collect(), edges)
}

fn handle_puzzle1(raw: &str) -> usize {
    let (steps, edges) = parse(raw);
    let mut c = "AAA";
    let mut num_steps = 0;
    let mut steps = steps.into_iter().cycle();
    while let Some(step) = steps.next()
        && c != "ZZZ"
    {
        if let Some(edge) = edges.get(c) {
            c = if step == 'L' { edge.0 } else { edge.1 };
            num_steps += 1;
        } else {
            println!("I don't have an edge named {c}");
        }
    }

    num_steps
}

fn handle_puzzle2(raw: &str) -> i64 {
    todo!()
}

#[test]
fn test_puzzle1() {
    let test_input = r#""#;

    assert_eq!(handle_puzzle1(test_input), todo!());
}

#[test]
fn test_puzzle2() {
    let test_input = r#""#;

    assert_eq!(handle_puzzle2(test_input), todo!());
}

#[test]
fn test_parse() {
    let test_input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    let (steps, edges) = parse(&test_input);

    assert!(edges.contains_key("AAA"));
    assert!(edges.contains_key("BBB"));
}
