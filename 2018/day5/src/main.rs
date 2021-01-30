fn main() {
    let input = include_str!("input.txt").trim();
    dbg!(chain_react(&input));
    dbg!(remove_units(&input));
}

fn remove_units(polymer_str: &str) -> usize {
    let mut shortest = polymer_str.len();
    for c in b'A'..=b'Z' {
        let unit1 = c as char;
        let unit2 = (c + 32) as char;
        let removed_polymer = polymer_str.replace(unit1, "").replace(unit2, "");
        let reacted_polymer_length = chain_react(&removed_polymer);
        if shortest > reacted_polymer_length {
            shortest = reacted_polymer_length;
        }
    }
    shortest
}

fn chain_react(polymer_str: &str) -> usize {
    let mut polymer = polymer_str.as_bytes().to_vec();
    let mut reacted_polymer = Vec::new();
    loop {
        let mut reacted = false;
        let mut i = 0;
        while i < polymer.len() - 1 {
            if react(polymer[i], polymer[i + 1]) {
                reacted = true;
                i += 2;
                continue;
            }
            reacted_polymer.push(polymer[i]);
            i += 1;
        }
        if i == polymer.len() - 1 {
            reacted_polymer.push(polymer[i]);
        }
        std::mem::swap(&mut polymer, &mut reacted_polymer);
        reacted_polymer.clear();
        if !reacted {
            break;
        }
    }
    polymer.len()
}

fn react(b1: u8, b2: u8) -> bool {
    (b1 as i16 - b2 as i16).abs() == 32
}
