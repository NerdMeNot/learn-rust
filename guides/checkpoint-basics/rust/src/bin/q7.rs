fn sum_below(n: usize) -> usize {
    (0..=n - 1).sum()
}

fn main() {
    println!("{}", sum_below(4));
    println!("{}", sum_below(0));
}
