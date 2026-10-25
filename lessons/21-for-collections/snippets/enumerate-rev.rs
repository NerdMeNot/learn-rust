fn main() {
    let names = ["ana", "bo", "cy"];

    for (i, n) in names.iter().enumerate().rev() {
        println!("enumerate().rev(): {i} is {n}");
    }

    for (i, n) in names.iter().rev().enumerate() {
        println!("rev().enumerate(): {i} is {n}");
    }
}
