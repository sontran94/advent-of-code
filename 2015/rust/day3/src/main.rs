fn main() {
    let directions = include_str!("input.txt")
        .trim()
        .as_bytes()
        .iter()
        .map(|&v| parse_direction(v))
        .collect::<Vec<Direction>>();
    println!("Santa can visit {} houses", one_worker(&directions));
    println!(
        "Santa and Robo-Santa can visit {} houses",
        two_workers(&directions)
    );
}

fn one_worker(directions: &[Direction]) -> usize {
    let mut pos_history = vec![Pos::default()];
    let mut current_pos = Pos::default();
    for d in directions {
        current_pos = Pos::new(&current_pos, d);
        if !pos_history.contains(&current_pos) {
            pos_history.push(current_pos.clone());
        }
    }
    pos_history.len()
}

fn two_workers(directions: &[Direction]) -> usize {
    let mut pos_history = vec![Pos::default()];
    let mut current_pos = vec![Pos::default(); 2];
    for (i, d) in directions.iter().enumerate() {
        if i % 2 == 0 {
            current_pos[0] = Pos::new(&current_pos[0], d);
            if !pos_history.contains(&current_pos[0]) {
                pos_history.push(current_pos[0].clone());
            }
        } else {
            current_pos[1] = Pos::new(&current_pos[1], d);
            if !pos_history.contains(&current_pos[1]) {
                pos_history.push(current_pos[1].clone());
            }
        }
    }
    pos_history.len()
}

#[derive(Default, Debug, Eq, PartialEq, Hash, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(prev: &Pos, go: &Direction) -> Self {
        let mut new_pos = Self {
            x: prev.x,
            y: prev.y,
        };
        match go {
            Direction::Left => new_pos.x -= 1,
            Direction::Right => new_pos.x += 1,
            Direction::Up => new_pos.y += 1,
            Direction::Down => new_pos.y -= 1,
        }
        new_pos
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn parse_direction(s: u8) -> Direction {
    if s == b'<' {
        Direction::Left
    } else if s == b'>' {
        Direction::Right
    } else if s == b'^' {
        Direction::Up
    } else {
        Direction::Down
    }
}
