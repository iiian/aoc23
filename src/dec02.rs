use std::{error::Error, fs::read_to_string};

use regex::Regex;
fn main() -> Result<(), Box<dyn Error>> {
    let raw = read_to_string("inputs/dec_02.txt")?;

    let result = handle_puzzle1(&raw);
    println!("Test 1: {result}");

    let result = handle_puzzle2(&raw);
    println!("Test 2: {result}");
    Ok(())
}
struct Event {
    blue: u64,
    green: u64,
    red: u64,
}

impl Event {
    pub fn new(blue: u64, green: u64, red: u64) -> Self {
        Self { blue, green, red }
    }
}

fn handle_puzzle1(raw: &str) -> u64 {
    let max: Event = Event::new(14, 13, 12);
    let id_rgx = Regex::new(r"Game (\d+)").unwrap();
    let subevent_rgx = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    raw.lines()
        .map(|line| {
            line.split_once(": ")
                .map(|(game, events)| {
                    let game = id_rgx.captures(game).unwrap().get(1).unwrap().as_str();
                    let events = events.split("; ").map(|event| {
                        event
                            .split(", ")
                            .fold(Event::new(0, 0, 0), |mut event, subevent| {
                                let subevent_cap = subevent_rgx.captures(subevent).unwrap();
                                let subevent_count = subevent_cap.get(1).unwrap().as_str();
                                let subevent_color = subevent_cap.get(2).unwrap().as_str();
                                match subevent_color {
                                    "blue" => event.blue = subevent_count.parse().unwrap(),
                                    "green" => event.green = subevent_count.parse().unwrap(),
                                    "red" => event.red = subevent_count.parse().unwrap(),
                                    _ => unreachable!(),
                                };

                                event
                            })
                    });
                    (game.parse::<u64>().unwrap(), events)
                })
                .unwrap()
        })
        .filter(|(_, events)| {
            events.clone().all(|Event { blue, green, red }| {
                blue <= max.blue && green <= max.green && red <= max.red
            })
        })
        .map(|(id, _)| id)
        .reduce(|a, b| a + b)
        .unwrap()
}

fn handle_puzzle2(raw: &str) -> u64 {
    let fact_rgx = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    raw.lines()
        .map(|line| {
            line.split_once(": ")
                .map(|(_, game_events)| {
                    game_events
                        .split("; ")
                        .map(|event_facts| {
                            event_facts.split(", ").map(|fact| {
                                let cap = fact_rgx.captures(fact).unwrap();
                                let count = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
                                let color = cap.get(2).unwrap().as_str();

                                (count, color)
                            })
                        })
                        .fold(Event::new(0, 0, 0), |power, facts| {
                            facts.fold(power, |mut power, (count, color)| {
                                match color {
                                    "blue" => power.blue = power.blue.max(count),
                                    "green" => power.green = power.green.max(count),
                                    "red" => power.red = power.red.max(count),
                                    _ => unreachable!(),
                                }

                                power
                            })
                        })
                })
                .map(|Event { blue, green, red }| blue * green * red)
                .unwrap()
        })
        .sum::<u64>()
}

#[test]
fn test_puzzle1() {
    let test_input = r#""Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    assert_eq!(handle_puzzle1(test_input), 8);
}

#[test]
fn test_puzzle2() {
    let test_input = r#""Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    assert_eq!(handle_puzzle2(test_input), 2286);
}
