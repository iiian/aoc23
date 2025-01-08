use std::collections::{HashSet, VecDeque};
use std::time::Instant;
use std::{error::Error, fs::read_to_string};
fn main() -> Result<(), Box<dyn Error>> {
    let raw = read_to_string("inputs/dec10.txt")?;

    let now = Instant::now();
    let result = handle_puzzle1(&raw, [Dir::N, Dir::S]);

    println!("Test 1: {result}, {}µs", now.elapsed().as_millis());

    let now = Instant::now();
    let result = handle_puzzle2(&raw);
    println!("Test 2: {result}, {}µs", now.elapsed().as_millis());
    Ok(())
}

fn parse(input: &str) -> ((usize, usize), Vec<Vec<char>>) {
    let mut out = vec![];
    let mut start: (usize, usize) = (usize::MAX, usize::MAX);
    for (i, line) in input.lines().enumerate() {
        out.push(vec![]);
        for (j, char) in line.char_indices() {
            if char == 'S' {
                start = (i, j);
            }
            out[i].push(char);
        }
    }
    assert_ne!(start, (usize::MAX, usize::MAX));
    (start, out)
}

#[derive(PartialEq, Clone, Copy)]
enum Dir {
    N,
    S,
    W,
    E,
}
fn handle_puzzle1(raw: &str, starting_directions: [Dir; 2]) -> usize {
    let (start, grid) = parse(raw);
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut frontier = VecDeque::from([(start, Dir::N)]);
    let mut dist = 0_usize;
    while let Some((next @ (i, j), direction)) = frontier.pop_front() {
        if visited.contains(&next) {
            break;
        }
        visited.insert(next);
        dist += 1;
        match &grid[i][j] {
            'S' => {
                // from a soft analysis, I know that it must be N & S
                for direction in &starting_directions {
                    frontier.push_back(match direction {
                        Dir::N => ((i - 1, j), *direction),
                        Dir::S => ((i + 1, j), *direction),
                        Dir::E => ((i, j + 1), *direction),
                        Dir::W => ((i, j - 1), *direction),
                    });
                }
            }
            '|' => {
                frontier.push_back(if direction == Dir::N {
                    ((i - 1, j), Dir::N)
                } else {
                    ((i + 1, j), Dir::S)
                });
            }
            '-' => {
                frontier.push_back(if direction == Dir::E {
                    ((i, j + 1), Dir::E)
                } else {
                    ((i, j - 1), Dir::W)
                });
            }
            'J' => {
                frontier.push_back(if direction == Dir::S {
                    ((i, j - 1), Dir::W)
                } else {
                    ((i - 1, j), Dir::N)
                });
            }
            'L' => {
                frontier.push_back(if direction == Dir::S {
                    ((i, j + 1), Dir::E)
                } else {
                    ((i - 1, j), Dir::N)
                });
            }
            'F' => {
                frontier.push_back(if direction == Dir::W {
                    ((i + 1, j), Dir::S)
                } else {
                    ((i, j + 1), Dir::E)
                });
            }
            '7' => {
                frontier.push_back(if direction == Dir::E {
                    ((i + 1, j), Dir::S)
                } else {
                    ((i, j - 1), Dir::W)
                });
            }
            _ => unreachable!(),
        }
    }

    dist / 2
}

fn handle_puzzle2(raw: &str) -> i64 {
    todo!()
}

#[test]
fn test_puzzle1() {
    let test_input = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

    assert_eq!(handle_puzzle1(test_input, [Dir::S, Dir::E]), 4);

    let test_input = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

    assert_eq!(handle_puzzle1(test_input, [Dir::S, Dir::E]), 8);
}

#[test]
fn test_puzzle2() {
    let test_input = r#""#;

    assert_eq!(handle_puzzle2(test_input), todo!());
}
