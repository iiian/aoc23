use core::str;
use std::{collections::{HashMap, HashSet}, error::Error, fs::read_to_string};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let raw = read_to_string("inputs/dec03.txt")?;

    let result = handle_puzzle1(&raw);
    println!("Test 1: {result}");

    let result = handle_puzzle2(&raw);
    println!("Test 2: {result}");
    Ok(())
}

fn is_tag(matrix: &[&[u8]], i: usize, j: usize) -> bool {
    if i >= matrix.len() {
        return false;
    }
    if j >= matrix[0].len() {
        return false;
    }
    is_tg(&matrix[i][j])
}

fn is_tg(ch: &u8) -> bool {
    !matches!(ch, b'0'..=b'9' | b'.')
}

/// Check to the left (including diagonal) of (i, j)
fn is_assoc1(matrix: &[&[u8]], i: usize, j: usize) -> bool {
    if i > 0 && j > 0 && is_tag(matrix, i - 1, j - 1) {
        return true;
    }
    if j > 0 && (is_tag(matrix, i, j - 1) || is_tag(matrix, i + 1, j - 1)) {
        return true;
    }
    if is_assoc2(matrix, i, j) {
        return true;
    }
    false
}

/// Check above and below (i,j)
fn is_assoc2(matrix: &[&[u8]], i: usize, j: usize) -> bool {
    if i > 0 && is_tag(matrix, i - 1, j) {
        return true;
    }
    is_tag(matrix, i + 1, j)
}

fn handle_puzzle1(raw: &str) -> i64 {
    let matrix = raw.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();

    let mut part_numbers = Vec::<i64>::new();

    // (jstart, jend, has_a_tag_symbol_nearby)
    let mut range = Option::<(usize, usize, bool)>::None;

    for (i, line) in matrix.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            match c {
                b'0'..=b'9' => {
                    if range.is_none() {
                        // new number
                        range = Some((j, j, is_assoc1(&matrix, i, j)));
                    } else if let Some((_, ref mut jend, ref mut is_assoc)) = range {
                        // number continuation
                        *is_assoc = *is_assoc || is_assoc2(&matrix, i, j);
                        *jend = j;
                    }
                }
                x => {
                    // number end
                    if let Some((jst, jen, absolutely_assoc)) = range {
                        if absolutely_assoc || is_assoc2(&matrix, i, j) || is_tg(x) {
                            let part_number = str::from_utf8(&matrix[i][jst..=jen])
                                .unwrap()
                                .parse::<i64>()
                                .unwrap();
                            part_numbers.push(part_number);
                        }
                    }
                    range = None;
                }
            }
        }
        // number end
        if let Some((jst, jen, absolutely_assoc)) = range {
            if absolutely_assoc {
                let part_number = str::from_utf8(&matrix[i][jst..=jen])
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();
                part_numbers.push(part_number);
            }
        }
        range = None;
    }
    part_numbers.into_iter().sum()
}

fn handle_puzzle2(raw: &str) -> i64 {
    // we're gonna do a find gears, span numbers that grow off of it.
    let matrix = raw.lines().map(|line| line.chars().collect::<String>()).collect::<Vec<_>>();
    let gear_re = Regex::new(r"\*").unwrap();
    let mut gears = HashMap::<(usize, usize), Vec<i64>>::new();

    for (i, row) in matrix.iter().enumerate() {
        for cap in gear_re.captures_iter(row) {
            gears.insert((i, cap.get(0).unwrap().start()), vec![]);
        }
    }


    let num_re = Regex::new(r"\d+").unwrap();
    for (i, row) in matrix.iter().enumerate() {
        for cap in num_re.captures_iter(row) {
            let Some(m) = cap.get(0) else {panic!()};

            for r in i.max(1)-1..i.min(matrix.len()-2)+2 {
                for c in m.start().max(1)-1..m.end()+1 {
                    if gears.contains_key(&(r, c)) {
                        gears.entry((r, c)).and_modify(|d| {
                            let num = m.as_str().parse::<i64>().unwrap();
                            d.push(num);
                        });
                    }
                }
            }
        }
    }

    gears.iter().filter_map(|(_, subratios)| {
        if subratios.len() == 2 {
            return Some(subratios[0] * subratios[1]);
        }
        None
    }).sum()
}


#[test]
fn test_puzzle1() {
    let test_input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    assert_eq!(handle_puzzle1(test_input), 4361);
}

#[test]
fn test_puzzle2() {
    let test_input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    assert_eq!(handle_puzzle2(test_input), 467835);

    let test_input = r#"..................
..................
1.1.1...1..1..1...
.*...*...*..*1.*..
......1.1......1..
1..11.............
1*..*...16........
.........*24......
..........2.......
..................
..................
..................
.................."#;

    assert_eq!(handle_puzzle2(test_input), 6);
}
