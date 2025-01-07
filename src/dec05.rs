// use std::{error::Error, fs::read_to_string, ops::Range, sync::{Arc, Mutex}, time::Instant};

// use rayon::iter::{ParallelBridge, ParallelIterator};

// fn main() -> Result<(), Box<dyn Error>> {
//     let raw = read_to_string("inputs/dec05.txt")?;

//     let now = Instant::now();
//     let result = handle_puzzle1(&raw);

//     println!("Test 1: {result}, {}µs", now.elapsed().as_millis());

//     let now = Instant::now();
//     let result = handle_puzzle2(&raw);
//     println!("Test 2: {result}, {}µs", now.elapsed().as_millis());
//     Ok(())
// }

// fn handle_puzzle1(raw: &str) -> usize {
//     let (seeds, maps) = parse(raw);

//     let mut min_location = usize::MAX;
//     for mut entity in seeds {
//         for map in &maps {
//             for (dest, src, range) in map {
//                 if (*src..*src + *range).contains(&entity) {
//                     entity = dest + (entity - src);
//                     break;
//                 }
//             }
//         }

//         min_location = min_location.min(entity);
//     }

//     min_location
// }

// fn handle_puzzle2(raw: &str) -> usize {
//     let (seeds, maps) = parse(raw);

//     std::collections::RangeSet
// }

// type EntityMap = Vec<Vec<(usize, usize, usize)>>;
// type Workload = (Vec<usize>, EntityMap);

// fn parse(raw: &str) -> Workload {
//     let mut chunks = raw.split("\n\n");

//     let seeds = chunks
//         .next()
//         .unwrap()
//         .split_once(": ")
//         .unwrap()
//         .1
//         .split_whitespace()
//         .map(|num| num.parse::<usize>().unwrap())
//         .collect::<Vec<_>>();

//     let maps = chunks
//         .map(|chunk| {
//             chunk
//                 .lines()
//                 .skip(1)
//                 .map(|line| {
//                     let mut nums = line
//                         .split_whitespace()
//                         .map(|num| num.parse::<usize>().unwrap());
//                     (
//                         nums.next().unwrap(),
//                         nums.next().unwrap(),
//                         nums.next().unwrap(),
//                     )
//                 })
//                 .collect()
//         })
//         .collect::<Vec<Vec<_>>>();

//     (seeds, maps)
// }

// #[test]
// fn test_puzzle1() {
//     let test_input = r#"seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2
// 52 50 48

// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15

// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4

// water-to-light map:
// 88 18 7
// 18 25 70

// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13

// temperature-to-humidity map:
// 0 69 1
// 1 0 69

// humidity-to-location map:
// 60 56 37
// 56 93 4"#;

//     assert_eq!(handle_puzzle1(test_input), todo!());
// }

// #[test]
// fn test_puzzle2() {
//     let test_input = r#""#;

//     assert_eq!(handle_puzzle2(test_input), todo!());
// }

// #[derive(Debug, Clone)]
// pub struct RangeSet {
//     ranges: Vec<Range<i64>>,
// }

// impl RangeSet {
//     pub fn new(ranges: Vec<Range<i64>>) -> Self {
//         RangeSet { ranges }
//     }

//     // union is inplace for performance reasons
//     pub fn union(&mut self, other: Self) {
//         match (self.ranges.is_empty(), other.ranges.is_empty()) {
//             (true, true) => self.ranges.clear(),
//             (true, false) => self.ranges = other.ranges,
//             (false, true) => (),
//             _ => {
//                 self.ranges.extend(other.ranges);
//                 self.ranges.sort_by(|a, b| a.start.cmp(&b.start));

//                 let mut result = Vec::new();
//                 let mut current = self.ranges[0].clone();

//                 for range in self.ranges[1..].iter() {
//                     if current.end < range.start {
//                         result.push(current);
//                         current = range.clone();
//                     } else {
//                         current.end = current.end.max(range.end);
//                     }
//                 }
//                 result.push(current);

//                 self.ranges = result;
//             }
//         }
//     }

//     pub fn intersection(&self, other: &Self) -> Self {
//         let mut result = Vec::new();
//         for range1 in self.ranges.iter() {
//             for range2 in other.ranges.iter() {
//                 if range1.start < range2.end && range1.end > range2.start {
//                     let start = range1.start.max(range2.start);
//                     let end = range1.end.min(range2.end);
//                     result.push(start..end);
//                 }
//             }
//         }
//         RangeSet { ranges: result }
//     }

//     pub fn difference(&self, other: &Self) -> Self {
//         let mut result = self.ranges.clone();
//         for other_range in other.ranges.iter() {
//             let mut new_result = Vec::new();
//             for range in result.iter() {
//                 if range.start >= other_range.end || range.end <= other_range.start {
//                     new_result.push(range.clone());
//                 } else {
//                     if range.start < other_range.start {
//                         new_result.push(range.start..other_range.start);
//                     }
//                     if range.end > other_range.end {
//                         new_result.push(other_range.end..range.end);
//                     }
//                 }
//             }
//             result = new_result;
//         }
//         RangeSet { ranges: result }
//     }

//     pub fn get_first(&self) -> Option<i64> {
//         self.ranges.first().map(|r| r.start)
//     }

//     pub fn iter(&self) -> std::vec::IntoIter<Range<i64>> {
//         self.ranges.clone().into_iter()
//     }
// }
