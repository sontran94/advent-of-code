fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt")
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<Wrap>, _>>()?;

    let area: u32 = input.iter().map(|v| v.wrap_area()).sum();
    dbg!(area);

    let ribbon: u32 = input.iter().map(|v| v.rippon_length()).sum();
    dbg!(ribbon);

    Ok(())
}

#[derive(Debug)]
struct Wrap {
    length: u32,
    width: u32,
    height: u32,
}

impl Wrap {
    fn wrap_area(&self) -> u32 {
        let area_1 = self.length * self.width;
        let area_2 = self.length * self.height;
        let area_3 = self.width * self.height;
        let mut min = std::cmp::min(area_1, area_2);
        min = std::cmp::min(min, area_3);
        2 * (area_1 + area_2 + area_3) + min
    }

    fn rippon_length(&self) -> u32 {
        let length_1 = 2 * (self.length + self.width);
        let length_2 = 2 * (self.length + self.height);
        let length_3 = 2 * (self.width + self.height);
        let mut min = std::cmp::min(length_1, length_2);
        min = std::cmp::min(min, length_3);
        min + self.length * self.width * self.height
    }
}

fn parse_line(s: &str) -> anyhow::Result<Wrap> {
    peg::parser! {
      grammar parser() for str {
        rule number() -> u32
          = n:$(['0'..='9']+) { n.parse().unwrap() }

        pub(crate) rule line() -> Wrap
          = length:number() "x" width:number() "x" height:number(){
              Wrap { length, width, height }
          }
      }
    }
    Ok(parser::line(s)?)
}

#[test]
fn test_ribbon() {
    let w = Wrap {
        length: 2,
        width: 3,
        height: 4,
    };
    assert_eq!(w.rippon_length(), 34);
}
