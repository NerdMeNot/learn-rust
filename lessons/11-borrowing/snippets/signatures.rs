fn largest(xs: &[i32]) -> i32 {
    let mut best = xs[0];
    for x in xs {
        if *x > best {
            best = *x;
        }
    }
    best
}

fn is_greeting(text: &str) -> bool {
    text.starts_with("hello")
}

// This function makes something new, so it returns an owned String.
fn shout(text: &str) -> String {
    let mut out = text.to_uppercase();
    out.push('!');
    out
}

fn main() {
    let list = vec![4, 9, 2];
    let array = [7, 1, 8];
    println!("largest(&list): {}", largest(&list));
    println!("largest(&array): {}", largest(&array));
    println!("largest(&list[1..]): {}", largest(&list[1..]));

    let owned = String::from("hello world");
    println!("is_greeting(&owned): {}", is_greeting(&owned));
    println!("is_greeting(\"goodbye\"): {}", is_greeting("goodbye"));
    println!("is_greeting(&owned[..5]): {}", is_greeting(&owned[..5]));

    let loud = shout("hello");
    println!("shout(\"hello\"): {loud}");
    println!("owned is still ours: {owned}");
}
