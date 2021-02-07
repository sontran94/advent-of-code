fn main() {
    let key = "ckczppom";
    print!("Secret number: {}", find_number_n_zeroes(&key, 6));
}

fn find_number_n_zeroes(key: &str, n: usize) -> u128 {
    let mut number: u128 = 1;
    let mut hash: String;
    loop {
        hash = format!("{:x}", md5::compute(format!("{}{}", key, number)));
        if hash[0..n].chars().all(|c| c == '0') {
            return number;
        }
        number += 1;
    }
}
