use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    // TODO! ---------------------------__
    //                                  __
    let raw = read_to_string("inputs/dec??.txt")?;

    let now = Instant::now();
    let result = handle_puzzle1(&raw);

    println!("Test 1: {result}, {}µs", now.elapsed().as_millis());

    let now = Instant::now();
    let result = handle_puzzle2(&raw);
    println!("Test 2: {result}, {}µs", now.elapsed().as_millis());
    Ok(())
}

fn handle_puzzle1(raw: &str) -> i64 {
    todo!()
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
