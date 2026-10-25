fn main() {
    let mut factorial: u128 = 1;
    for n in 1..=40_u128 {
        match factorial.checked_mul(n) {
            Some(next) => factorial = next,
            None => {
                println!(
                    "{n}! does not fit in a u128; {} is the last that does",
                    n - 1
                );
                break;
            }
        }
    }
    println!("{factorial}");
    println!("u128::MAX: {}", u128::MAX);
}
