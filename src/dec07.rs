use phf::phf_map;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use std::{error::Error, fs::read_to_string};
fn main() -> Result<(), Box<dyn Error>> {
    // TODO! ---------------------------__
    //                                  __
    let raw = read_to_string("inputs/dec07.txt")?;

    let now = Instant::now();
    let result = handle_puzzle1(&raw);

    println!("Test 1: {result}, {}µs", now.elapsed().as_millis());

    let now = Instant::now();
    let result = handle_puzzle2(&raw);
    println!("Test 2: {result}, {}µs", now.elapsed().as_millis());
    Ok(())
}

#[derive(PartialEq, PartialOrd, Eq)]
struct Hand<'h> {
    cards: &'h str,
}

impl<'h> Ord for Hand<'h> {
    fn cmp(&self, other: &Self) -> Ordering {
        let id_self = self.identify();
        let id_other = other.identify();
        match id_self.cmp(&id_other) {
            Ordering::Equal => {
                let self_chars = self.cards.chars();
                let other_chars = other.cards.chars();

                for (self_char, other_char) in self_chars.zip(other_chars) {
                    let cmp = Hand::rank(&self_char).cmp(&Hand::rank(&other_char));
                    if !cmp.is_eq() {
                        return cmp;
                    }
                }
                
                Ordering::Equal
            },
            x => x,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HIGH = 0,
    ONEP = 1,
    TWOP = 2,
    THRE = 4,
    FULL = 8,
    FOUR = 16,
    FIVE = 32,
}

impl<'h> Hand<'h> {
    pub fn new(cards: &'h str) -> Self {
        Self { cards }
    }

    fn rank(ch: &char) -> usize {
        match ch {
            'A' => 1 << 14,
            'K' => 1 << 13,
            'Q' => 1 << 12,
            'J' => 1 << 11,
            'T' => 1 << 10,
            '2'..='9' => 1 << ch.to_digit(10).unwrap() as usize,
            _ => unreachable!(),
        }
    }

    pub fn identify(&self) -> HandType {
        let hand = self.cards.chars().fold(HashMap::new(), |mut acc, next| {
            *acc.entry(next).or_default() += 1_u8;
            acc
        });
        let counts = hand.iter().fold(
            HashMap::<u8, HashSet<char>>::new(),
            |mut acc, (card, count)| {
                acc.entry(*count).or_default().insert(*card);
                acc
            },
        );
        if counts.contains_key(&5) {
            return HandType::FIVE;
        }
        if counts.contains_key(&4) {
            return HandType::FOUR;
        }
        if let Some(three) = counts.get(&3) {
            if counts.contains_key(&2) {
                return HandType::FULL;
            }
            return HandType::THRE;
        }
        if let Some(twos) = counts.get(&2) {
            if twos.len() == 2 {
                return HandType::TWOP;
            }
            return HandType::ONEP;
        }

        HandType::HIGH
    }
}

fn handle_puzzle1(raw: &str) -> usize {
    let mut hands = raw
        .lines()
        .map(|line| {
            let (hand, bet) = line.split_once(' ').unwrap();
            let hand = Hand::new(hand);

            (hand, bet.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    hands.sort_by(|(a,_), (b,_)| a.cmp(b));
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bet))| {
            let value = (i + 1) * bet;
            println!("{value}");
            value
        })
        .sum()
}

fn handle_puzzle2(raw: &str) -> i64 {
    todo!()
}

#[test]
fn test_puzzle1() {
    let test_input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    assert_eq!(handle_puzzle1(test_input), 6440);
}

#[test]
fn test_puzzle2() {
    let test_input = r#""#;

    assert_eq!(handle_puzzle2(test_input), todo!());
}
