fn main() {
    for n in [-4, 0, 7, 12] {
        let what = match n {
            n if n < 0 => "negative",
            0 => "zero",
            n if n % 2 == 0 => "even",
            _ => "odd",
        };
        println!("{n} is {what}");
    }
}
