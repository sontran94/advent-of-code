use anyhow::{anyhow, Result};
use regex::Regex;
fn main() {
    let input = include_str!("input.txt").trim();
    if let Ok([players, marbles]) = parse_input(input) {
        dbg!(players, marbles);
    }
}

fn parse_input(input: &str) -> Result<[usize; 2]> {
    let re = Regex::new(r"([[:digit:]]+).*\s([[:digit:]]+)").unwrap();
    match re.captures(input) {
        None => Err(anyhow!("Missing numbers")),
        Some(numbers) => Ok([
            numbers[1].parse::<usize>().unwrap(),
            numbers[2].parse::<usize>().unwrap(),
        ]),
    }
}
