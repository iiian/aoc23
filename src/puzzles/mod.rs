use std::error::Error;

mod dec01;

#[derive(Debug)]
pub struct UnimplementedError;

impl std::fmt::Display for UnimplementedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unimplemented")
    }
}
impl std::error::Error for UnimplementedError {}

pub fn run_puzzle(day: u8, id: u8, input: &str) -> Result<(), Box<dyn Error>> {
    let ans = match (day, id) {
        (1, 1) => dec01::puzzle1(input),
        (1, 2) => dec01::puzzle2(input),
        _ => Err(Box::new(UnimplementedError) as Box<dyn Error>),
    }?;

    println!("Day {}, Puzzle {} -- {}", day, id, ans);

    Ok(())
}
