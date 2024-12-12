use std::time::Instant;
use std::{error::Error, fs::read_to_string};

use itertools::Itertools;
fn main() -> Result<(), Box<dyn Error>> {
    // TODO! ---------------------------__
    //                                  __
    let raw = read_to_string("inputs/dec06.txt")?;

    let now = Instant::now();
    let result = handle_puzzle1(&raw);

    println!("Test 1: {result}, {}µs", now.elapsed().as_millis());

    let now = Instant::now();
    let result = handle_puzzle2(&raw);
    println!("Test 2: {result}, {}µs", now.elapsed().as_millis());
    Ok(())
}

type PuzzleInput = Vec<(usize, usize)>;

fn parse_puzzle1(raw: &str) -> PuzzleInput {
    let mut lines = raw.lines();
    let line_1 = lines.next().unwrap();
    let line_2 = lines.next().unwrap();

    let (_, time) = line_1.split_once(":").unwrap();
    let (_, distance) = line_2.split_once(":").unwrap();

    let time = time.split_whitespace().map(|num| num.parse().unwrap());
    let distance = distance.split_whitespace().map(|num| num.parse().unwrap());

    time.zip(distance).collect()
}

fn handle_puzzle1(raw: &str) -> i64 {
    let races = parse_puzzle1(raw);

    races
        .into_iter()
        .map(|(racetime, best_distance)| {
            // through a bit of velocity x time = distance
            // and good old trusty quadratic formula,
            // I think the range of race beating values are bound by
            // t = (racetime +- sqrt(racetime**2 - 4*best_distance)) / 2.
            //
            // That formula provides two poles that satisfy the constraint equation
            // - (t**2) + racetime - best_distance > 0,
            // which is a reformulation of boat_velocity x time_left_to_run > best_distance.
            let racetime = racetime as f32;
            let best_distance = best_distance as f32;
            let lo = ((racetime + f32::sqrt(racetime.powf(2_f32) - 4_f32 * best_distance)) / 2_f32)
                as i64;
            let hi = ((racetime - f32::sqrt(racetime.powf(2_f32) - 4_f32 * best_distance)) / 2_f32)
                as i64;

            assert!(lo > 0);
            assert!(hi > 0);
            hi - lo
        })
        .reduce(|prod, next| prod * next)
        .unwrap()
}

fn parse_puzzle2(raw: &str) -> (usize, usize) {
    let mut lines = raw.lines();
    let line_1 = lines.next().unwrap();
    let line_2 = lines.next().unwrap();

    let (_, time) = line_1.split_once(":").unwrap();
    let (_, distance) = line_2.split_once(":").unwrap();

    let time = time
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let distance = distance
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    (time, distance)
}

fn handle_puzzle2(raw: &str) -> i64 {
    let (racetime, best_distance) = parse_puzzle2(raw);

    let racetime = racetime as f32;
    let best_distance = best_distance as f32;
    let lo = ((racetime + f32::sqrt(racetime.powf(2_f32) - 4_f32 * best_distance)) / 2_f32) as i64;
    let hi = ((racetime - f32::sqrt(racetime.powf(2_f32) - 4_f32 * best_distance)) / 2_f32) as i64;

    assert!(lo > 0);
    assert!(hi > 0);
    hi.abs_diff(lo).try_into().unwrap()
}

// #[test]
// fn test_puzzle1() {
//     let test_input = r#""#;
//
//     assert_eq!(handle_puzzle1(test_input), todo!());
// }

#[test]
fn test_puzzle2() {
    let test_input = r#"Time:      7  15   30
Distance:  9  40  200"#;

    assert_eq!(handle_puzzle2(test_input), 71503);
}
