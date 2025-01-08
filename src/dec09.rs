use std::time::Instant;
use std::{error::Error, fs::read_to_string};
fn main() -> Result<(), Box<dyn Error>> {
    let raw = read_to_string("inputs/dec09.txt")?;

    let now = Instant::now();
    let result = handle_puzzle1(&raw);

    println!("Test 1: {result}, {}µs", now.elapsed().as_millis());

    let now = Instant::now();
    let result = handle_puzzle2(&raw);
    println!("Test 2: {result}, {}µs", now.elapsed().as_millis());
    Ok(())
}

struct Seq {
    derivatives: Vec<i64>,
}

impl Seq {
    pub fn from(sequence: Vec<i64>) -> Self {
        let mut derivatives = vec![];

        for i in sequence {
            let mut curs = i;
            for deriv in derivatives.iter_mut() {
                let d = *deriv;
                *deriv = curs;
                curs -= d;
            }
            derivatives.push(curs);
        }

        return Seq { derivatives };
    }

    pub fn extrapolate(&self) -> i64 {
        self.derivatives.iter().sum()
    }
}

fn handle_puzzle1(raw: &str) -> i64 {
    parse(raw)
        .into_iter()
        .map(Seq::from)
        .map(|seq| seq.extrapolate())
        .sum()
}

fn handle_puzzle2(raw: &str) -> i64 {
    parse(raw)
        .into_iter()
        .map(|mut sequence| {
            sequence.reverse();
            Seq::from(sequence)
        })
        .map(|seq| seq.extrapolate())
        .sum()
}

fn parse(raw: &str) -> Vec<Vec<i64>> {
    raw.lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

#[test]
fn test_puzzle1() {
    let test_input = r#"0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45"#;

    assert_eq!(handle_puzzle1(test_input), 114);
}

#[test]
fn test_puzzle2() {
    let test_input = r#"0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45"#;

    assert_eq!(handle_puzzle2(test_input), 2);
}
