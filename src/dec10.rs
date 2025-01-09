use std::collections::{HashSet, VecDeque};
use std::time::Instant;
use std::{error::Error, fs::read_to_string};
fn main() -> Result<(), Box<dyn Error>> {
    let raw = read_to_string("inputs/dec10.txt")?;

    let now = Instant::now();
    let result = handle_puzzle1(&raw, [Dir::N, Dir::S]);

    println!("Test 1: {result}, {}µs", now.elapsed().as_micros());

    let now = Instant::now();
    let result = handle_puzzle2(&raw, '|', [Dir::N, Dir::S]);
    println!("Test 2: {result}, {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse(input: &str, symb: Option<char>) -> ((usize, usize), Vec<Vec<char>>) {
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

    // replace 'S' with it's corresponding symbol
    if let Some(symb) = symb {
        out[start.0][start.1] = symb;
    }

    (start, out)
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Dir {
    N,
    S,
    W,
    E,
}
fn handle_puzzle1(raw: &str, starting_directions: [Dir; 2]) -> usize {
    let (start, grid) = parse(raw, None);
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

fn go((i, j): (usize, usize), dir: Dir) -> ((usize, usize), Dir) {
    match dir {
        Dir::N => ((i - 1, j), dir),
        Dir::S => ((i + 1, j), dir),
        Dir::W => ((i, j - 1), dir),
        Dir::E => ((i, j + 1), dir),
    }
}
#[inline]
fn west((i, j): (usize, usize)) -> Option<(usize, usize)> {
    if j == 0 {
        None
    } else {
        Some((i, j - 1))
    }
}
#[inline]
fn east((i, j): (usize, usize)) -> Option<(usize, usize)> {
    Some((i, j + 1))
}
#[inline]
fn north((i, j): (usize, usize)) -> Option<(usize, usize)> {
    if i == 0 {
        None
    } else {
        Some((i - 1, j))
    }
}
#[inline]
fn south((i, j): (usize, usize)) -> Option<(usize, usize)> {
    Some((i + 1, j))
}
#[inline]
fn south_west((i, j): (usize, usize)) -> Option<(usize, usize)> {
    if j == 0 {
        None
    } else {
        Some((i + 1, j - 1))
    }
}
#[inline]
fn north_west((i, j): (usize, usize)) -> Option<(usize, usize)> {
    if i == 0 || j == 0 {
        None
    } else {
        Some((i - 1, j - 1))
    }
}
#[inline]
fn north_east((i, j): (usize, usize)) -> Option<(usize, usize)> {
    if i == 0 {
        None
    } else {
        Some((i - 1, j + 1))
    }
}
#[inline]
fn south_east((i, j): (usize, usize)) -> Option<(usize, usize)> {
    Some((i + 1, j + 1))
}

fn handle_puzzle2(raw: &str, symb: char, starting_directions: [Dir; 2]) -> usize {
    let (start, g) = parse(raw, Some(symb));
    let p = get_perimeter(&g, start, starting_directions);
    let mut ext = HashSet::<(usize, usize)>::new();
    // there should always be a leftmost, uppermost element. that element *must* be an 'F' kink, otherwise there would be an element further left or further up.
    // since we start on that element, as long as we remain oriented along the direction of the perimeter, the externality of our loop will always be on the left,
    // and the internality will be on the right.
    let start = p.iter().min_by_key(|k| **k).unwrap();
    let (mut c, mut dir) = (*start, Dir::N);
    let (imax, jmax) = (g.len() - 1, g[0].len() - 1);
    loop {
        match (g[c.0][c.1], dir) {
            ('|', Dir::N) => {
                fill(west(c), &mut ext, &p, imax, jmax);
                (c, dir) = go(c, Dir::N);
            }
            ('|', Dir::S) => {
                fill(east(c), &mut ext, &p, imax, jmax);
                (c, dir) = go(c, Dir::S);
            }
            ('-', Dir::E) => {
                fill(north(c), &mut ext, &p, imax, jmax);
                (c, dir) = go(c, Dir::E);
            }
            ('-', Dir::W) => {
                fill(south(c), &mut ext, &p, imax, jmax);
                (c, dir) = go(c, Dir::W);
            }
            ('F', Dir::N) => {
                fill(west(c), &mut ext, &p, imax, jmax);
                fill(north(c), &mut ext, &p, imax, jmax);
                fill(north_west(c), &mut ext, &p, imax, jmax);
                (c, dir) = go(c, Dir::E);
            }
            ('F', Dir::W) => {
                (c, dir) = go(c, Dir::S);
            }
            ('7', Dir::E) => {
                fill(north(c), &mut ext, &p, imax, jmax);
                fill(east(c), &mut ext, &p, imax, jmax);
                fill(north_east(c), &mut ext, &p, imax, jmax);
                (c, dir) = go(c, Dir::S);
            }
            ('7', Dir::N) => {
                (c, dir) = go(c, Dir::W);
            }
            ('L', Dir::W) => {
                fill(south(c), &mut ext, &p, imax, jmax);
                fill(west(c), &mut ext, &p, imax, jmax);
                fill(south_west(c), &mut ext, &p, imax, jmax);
                (c, dir) = go(c, Dir::N);
            }
            ('L', Dir::S) => {
                (c, dir) = go(c, Dir::E);
            }
            ('J', Dir::S) => {
                fill(south(c), &mut ext, &p, imax, jmax);
                fill(east(c), &mut ext, &p, imax, jmax);
                fill(south_east(c), &mut ext, &p, imax, jmax);
                (c, dir) = go(c, Dir::W);
            }
            ('J', Dir::E) => {
                (c, dir) = go(c, Dir::N);
            }
            (x, y) => {
                println!("{x}, {:?}", y);
                panic!();
            }
        }

        if c == *start {
            break;
        }
    }

    (g.len() * g[0].len()) - ext.len() - p.len()
}

fn fill(
    c: Option<(usize, usize)>,
    ext: &mut HashSet<(usize, usize)>,
    p: &HashSet<(usize, usize)>,
    imax: usize,
    jmax: usize,
) {
    if let Some(c @ (i, j)) = c {
        if ext.contains(&c) || p.contains(&c) || c.0 > imax || c.1 > jmax {
            return;
        }
        ext.insert(c);
        if i != 0 {
            fill(north(c), ext, p, imax, jmax);
        }
        if j != 0 {
            fill(east(c), ext, p, imax, jmax);
        }
        if i < imax {
            fill(south(c), ext, p, imax, jmax);
        }
        if j < jmax {
            fill(west(c), ext, p, imax, jmax);
        }
    }
}

fn get_perimeter(
    grid: &[Vec<char>],
    start: (usize, usize),
    starting_directions: [Dir; 2],
) -> HashSet<(usize, usize)> {
    let mut perimeter = HashSet::new();

    let mut frontier = VecDeque::from([(start, Dir::N)]);
    while let Some((next @ (i, j), direction)) = frontier.pop_front() {
        if perimeter.contains(&next) {
            break;
        }
        perimeter.insert(next);
        match &grid[i][j] {
            'S' => {
                // from a soft analysis, I know that it must be N & S
                for starting_direction in starting_directions {
                    frontier.push_back(match starting_direction {
                        Dir::N => ((i - 1, j), starting_direction),
                        Dir::S => ((i + 1, j), starting_direction),
                        Dir::E => ((i, j + 1), starting_direction),
                        Dir::W => ((i, j - 1), starting_direction),
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

    perimeter
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
    let test_input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

    assert_eq!(handle_puzzle2(test_input, '7', [Dir::W, Dir::S]), 10);
}
