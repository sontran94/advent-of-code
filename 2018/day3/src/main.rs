use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let claims = include_str!("input.txt")
        .trim()
        .split('\n')
        .map(parse_line)
        .collect::<Result<Vec<Claim>, _>>()?;

    let mut grid = HashMap::<[u16; 2], usize>::new();
    for claim in &claims {
        for [x, y] in claim.iter_points() {
            *grid.entry([x, y]).or_insert(0) += 1;
        }
    }

    dbg!(overlap(&grid));
    uniqe_claim(&claims, &grid)?;
    Ok(())
}

fn overlap(grid: &HashMap<[u16; 2], usize>) -> usize {
    grid.values().filter(|&&count| count > 1).count()
}

fn uniqe_claim(claims: &Vec<Claim>, grid: &HashMap<[u16; 2], usize>) -> anyhow::Result<()> {
    for claim in claims {
        if claim.iter_points().all(|p| grid[&p] == 1) {
            dbg!(claim.id);
            return Ok(());
        }
    }
    Err(anyhow::anyhow!("There is no uniqe claim"))
}

#[derive(PartialEq, Debug)]
struct Claim {
    id: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

impl Claim {
    fn iter_points(&self) -> IterPoints {
        IterPoints {
            claim: self,
            px: self.x,
            py: self.y,
        }
    }
}

struct IterPoints<'c> {
    claim: &'c Claim,
    px: u16,
    py: u16,
}

impl<'c> Iterator for IterPoints<'c> {
    type Item = [u16; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if self.py >= self.claim.y + self.claim.height {
            self.py = self.claim.y;
            self.px += 1;
        }
        if self.px >= self.claim.x + self.claim.width {
            return None;
        }
        let [px, py] = [self.px, self.py];
        self.py += 1;
        Some([px, py])
    }
}

fn parse_line(s: &str) -> anyhow::Result<Claim> {
    peg::parser! {
        grammar parser() for str {
            rule id() -> u16
                = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule x() -> u16
                = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule y() -> u16
                = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule width() -> u16
                = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule height() -> u16
                = n:$(['0'..='9']+) { n.parse().unwrap() }

            pub(crate) rule line() -> Claim
                = "#" id:id() " @ " x:x() "," y:y() ": " width:width() "x" height:height() {
                    Claim { id, x, y, width, height }
                    }
        }
    }
    Ok(parser::line(s)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_line("#15 @ 700,156: 28x29").unwrap(),
            Claim {
                id: 15,
                x: 700,
                y: 156,
                width: 28,
                height: 29,
            }
        );
    }

    #[test]
    fn test_overlap() {
        let input = vec![
            Claim {
                id: 1,
                x: 1,
                y: 3,
                width: 4,
                height: 4,
            },
            Claim {
                id: 2,
                x: 3,
                y: 1,
                width: 4,
                height: 4,
            },
            Claim {
                id: 3,
                x: 5,
                y: 5,
                width: 2,
                height: 2,
            },
        ];
        assert_eq!(overlap(&input), 4);
    }
}
