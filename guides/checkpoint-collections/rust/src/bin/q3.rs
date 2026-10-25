fn main() {
    let mut count = 0;
    let mut bump = || {
        count += 1;
        count
    };
    bump();
    bump();
    println!("{count}");

    let mut n = 10;
    let mut add = move || {
        n += 1;
        n
    };
    println!("{} {}", add(), add());
    println!("{n}");
}
