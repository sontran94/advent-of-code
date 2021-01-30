#![allow(dead_code, unused_variables)]
fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt").trim().split('\n');
    Ok(())
}

struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn from(s: &str) -> anyhow::Result<Self> {
        let parsed = s
            .split(',')
            .map(|val| val.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;
        Ok(Coordinate {
            x: parsed[0],
            y: parsed[1],
        })
    }

    fn distance(&self, other: &Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
