fn main() {
    let input = include_str!("input.txt").trim().as_bytes();
    let floor: i32 = input.iter().map(|&c| if c == b'(' { 1 } else { -1 }).sum();
    let basement = input
        .iter()
        .map(|&c| if c == b'(' { 1 } else { -1 })
        .scan(0, |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .position(|v| v == -1);
    println!("Santa arrives at floor number: {}", floor);
    if let Some(v) = basement {
        println!("The position of the basement is: {}", v + 1);
    } else {
        println!("Cannot get into the basement");
    }
}
