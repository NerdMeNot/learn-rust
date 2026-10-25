fn main() {
    let names = vec!["ana", "bo", "cy"];

    for (i, n) in names.iter().enumerate() {
        println!("enumerate: {i} is {n}");
    }

    for n in names.iter().rev() {
        println!("rev: {n}");
    }

    for n in &names[1..] {
        println!("slice [1..]: {n}");
    }

    let mut scores = vec![5, 5, 5];
    for (i, s) in scores.iter_mut().enumerate() {
        *s += i * 10;
    }
    println!("iter_mut().enumerate(): {scores:?}");
}
