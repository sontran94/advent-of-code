use lazy_static::lazy_static;

fn main() {
    let input = include_str!("input.txt");
    println!(
        "Number of nice strings part 1: {}",
        count_nice_string_1(&input, 3)
    );
    println!(
        "Number of nice strings part 2: {}",
        count_nice_string_2(&input)
    );
}

fn count_nice_string_1(input: &str, n: usize) -> usize {
    input
        .lines()
        .map(|line| check_nice_string_1(&line, n))
        .filter(|&v| v)
        .count()
}

fn count_nice_string_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| check_nice_string_2(&line))
        .filter(|&v| v)
        .count()
}

fn check_nice_string_1(s: &str, n: usize) -> bool {
    check_n_vowels(&s, n) && check_twice_in_row(&s) && check_no_appearance(&s)
}

fn check_nice_string_2(s: &str) -> bool {
    check_pair_in_row(&s) && check_repeat_one_between(&s)
}

fn check_n_vowels(s: &str, n: usize) -> bool {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"[aeiou]").unwrap();
    }
    RE.find_iter(&s).count() >= n
}

fn check_twice_in_row(s: &str) -> bool {
    lazy_static! {
        static ref RE: fancy_regex::Regex = fancy_regex::Regex::new(r"([[:alpha:]])\1").unwrap();
    }
    RE.is_match(&s).unwrap()
}

fn check_no_appearance(s: &str) -> bool {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"ab|cd|pq|xy").unwrap();
    }
    !RE.is_match(&s)
}

fn check_pair_in_row(s: &str) -> bool {
    lazy_static! {
        static ref RE: fancy_regex::Regex =
            fancy_regex::Regex::new(r"([[:alpha:]]{2}).*\1").unwrap();
    }
    RE.is_match(&s).unwrap()
}

fn check_repeat_one_between(s: &str) -> bool {
    lazy_static! {
        static ref RE: fancy_regex::Regex = fancy_regex::Regex::new(r"([[:alpha:]]).\1").unwrap();
    }
    RE.is_match(&s).unwrap()
}
#[test]
fn test_check_n_vowels() {
    assert_eq!(check_n_vowels("i", 1), true);
    assert_eq!(check_n_vowels("love", 2), true);
    assert_eq!(check_n_vowels("you", 3), false);
    assert_eq!(check_n_vowels("iloveyou", 4), true);
    assert_eq!(check_n_vowels("!", 0), true);
}

#[test]
fn test_check_twice_in_a_row() {
    assert_eq!(check_twice_in_row("ii"), true);
    assert_eq!(check_twice_in_row("ia"), false);
    assert_eq!(check_twice_in_row("iai"), false);
}

#[test]
fn test_check_no_appearance() {
    assert_eq!(check_no_appearance("abc"), false);
    assert_eq!(check_no_appearance("bce"), true);
    assert_eq!(check_no_appearance("xzy"), true);
    assert_eq!(check_no_appearance("pqxy"), false);
}

#[test]
fn test_check_nice_string_1() {
    assert_eq!(check_nice_string_1("ugknbfddgicrmopn", 3), true);
    assert_eq!(check_nice_string_1("jchzalrnumimnmhp", 3), false);
    assert_eq!(check_nice_string_1("haegwjzuvuyypxyu", 3), false);
}

#[test]
fn test_check_pair_in_a_row() {
    assert_eq!(check_pair_in_row("xyxy"), true);
    assert_eq!(check_pair_in_row("xxx"), false);
}

#[test]
fn test_check_repeat_one_between() {
    assert_eq!(check_repeat_one_between("xyxy"), true);
    assert_eq!(check_repeat_one_between("aaa"), true);
    assert_eq!(check_repeat_one_between("aai"), false);
}

#[test]
fn test_check_nice_string_2() {
    assert_eq!(check_nice_string_2("qjhvhtzxzqqjkmpb"), true);
    assert_eq!(check_nice_string_2("xxyxx"), true);
    assert_eq!(check_nice_string_2("uurcxstgmygtbstg"), false);
    assert_eq!(check_nice_string_2("ieodomkazucvgmuy"), false);
}
