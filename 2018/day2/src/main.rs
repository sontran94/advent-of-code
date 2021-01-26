use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<&str> = include_str!("input.txt")
        .trim()
        .split('\n')
        .collect::<Vec<_>>();
    checksum(&input);
    common_letters_id(&input)?;
    Ok(())
}

fn checksum(input: &[&str]) -> usize {
    let checked_pairs = input.iter().map(|s| check_appear(s));
    let twice: usize = checked_pairs.clone().filter(|pair| pair[0]).count();
    let three: usize = checked_pairs.filter(|pair| pair[1]).count();
    let check = twice * three;
    dbg!(check);
    check
}

fn check_appear(input: &str) -> Vec<bool> {
    let mut freq = HashMap::new();
    for c in input.chars() {
        let count = freq.entry(c).or_insert(0);
        *count += 1;
    }
    vec![
        freq.values().any(|&x| x == 2),
        freq.values().any(|&x| x == 3),
    ]
}

fn common_letters_id(input: &[&str]) -> Result<(), String> {
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if let Some(s) = common_letters(&input[i], &input[j]) {
                dbg!(s);
                return Ok(());
            }
        }
    }
    Err(String::from("Found no boxes"))
}

fn common_letters(id1: &str, id2: &str) -> Option<String> {
    let mut one_diff = false;
    for (c1, c2) in id1.chars().zip(id2.chars()) {
        if c1 != c2 {
            if one_diff {
                return None;
            }
            one_diff = true;
        }
    }
    Some(
        id1.chars()
            .zip(id2.chars())
            .filter(|&(c1, c2)| c1 == c2)
            .map(|(c, _)| c)
            .collect(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_appear() {
        let input = vec!["bababc", "abbcde", "ababab"];
        assert_eq!(check_appear(&input[0]), vec![true, true]);
        assert_eq!(check_appear(&input[1]), vec![true, false]);
        assert_eq!(check_appear(&input[2]), vec![false, true]);
    }
    #[test]
    fn test_checksum() {
        let input = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        assert_eq!(checksum(&input), 12);
    }
}
