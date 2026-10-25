fn main() {
    for n in [3, 5, 7, 15] {
        match (n % 3, n % 5) {
            (0, 0) => println!("{n}: FizzBuzz"),
            (0, _) => println!("{n}: Fizz"),
            (_, 0) => println!("{n}: Buzz"),
            _ => println!("{n}: {n}"),
        }
    }
}
