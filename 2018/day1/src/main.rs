use std::collections::HashSet;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<i32> = include_str!("input.txt")
        .trim()
        .split('\n')
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &[i32]) -> Result<(), Box<dyn Error>> {
    let s: i32 = input.iter().sum();
    dbg!(s);
    Ok(())
}

fn part2(input: &[i32]) -> Result<(), Box<dyn Error>> {
    let mut freq = 0;
    let mut seen = HashSet::new();
    seen.insert(freq);

    loop {
        for val in input {
            freq += val;
            if seen.contains(&freq) {
                dbg!(freq);
                return Ok(());
            }
            seen.insert(freq);
        }
    }
}
