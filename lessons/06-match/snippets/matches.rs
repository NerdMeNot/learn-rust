fn main() {
    let c = 'k';
    println!("lowercase: {}", matches!(c, 'a'..='z'));

    let age = 15;
    println!("teen: {}", matches!(age, 13..=19));
    println!(
        "odd and over 10: {}",
        matches!(age, n if n % 2 == 1 && n > 10)
    );

    let found = Some(4);
    println!("some even: {}", matches!(found, Some(n) if n % 2 == 0));
}
