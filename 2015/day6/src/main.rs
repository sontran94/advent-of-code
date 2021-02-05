fn main() -> anyhow::Result<()> {
    let instructions = include_str!("input.txt")
        .lines()
        .map(|line| parse_instructions(line))
        .collect::<Result<Vec<Instruction>, _>>()?;
    let mut grid = Grid::new(1000, 1000);
    grid.lit(&instructions);
    println!("Brightness level: {}", grid.lights.iter().sum::<usize>());
    Ok(())
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Light(usize, usize);

#[derive(Debug)]
struct Grid {
    row: usize,
    col: usize,
    lights: Vec<usize>,
}

impl Grid {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
            lights: vec![0; row * col],
        }
    }

    fn lit(&mut self, instructions: &[Instruction]) {
        for ins in instructions.iter() {
            match ins.action {
                Action::TurnOn => self.turn_on(&ins.from, &ins.to),
                Action::TurnOff => self.turn_off(&ins.from, &ins.to),
                Action::Toggle => self.toggle(&ins.from, &ins.to),
            }
        }
    }

    fn turn_on(&mut self, from: &Light, to: &Light) {
        for y in from.1..=to.1 {
            for x in from.0..=to.0 {
                self.lights[x + y * self.col] += 1;
            }
        }
    }
    fn turn_off(&mut self, from: &Light, to: &Light) {
        for y in from.1..=to.1 {
            for x in from.0..=to.0 {
                let index = x + y * self.col;
                self.lights[index] -= if self.lights[index] > 0 { 1 } else { 0 };
            }
        }
    }
    fn toggle(&mut self, from: &Light, to: &Light) {
        for y in from.1..=to.1 {
            for x in from.0..=to.0 {
                self.lights[x + y * self.col] += 2;
            }
        }
    }
}

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    from: Light,
    to: Light,
}

fn parse_instructions(s: &str) -> anyhow::Result<Instruction> {
    peg::parser! {
            grammar parser() for str {
                pub(crate) rule line() -> Instruction
                    = action:action() fx:number() "," fy:number()
                    " through " tx:number() "," ty:number()
                    { Instruction { action, from: Light(fx,fy), to: Light(tx,ty) }}

                rule number() -> usize
                    = n:$(['0'..='9']+) { n.parse().unwrap() }

                rule action() -> Action
                    = "turn on " { Action::TurnOn }
                    / "turn off " { Action::TurnOff }
                    / "toggle " { Action::Toggle }
        }
    }
    Ok(parser::line(s)?)
}
