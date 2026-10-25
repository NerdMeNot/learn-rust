fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn main() {
    println!("{}", sum(&[1, 2, 3]));
    println!("{}", sum(1, 2, 3));
}
