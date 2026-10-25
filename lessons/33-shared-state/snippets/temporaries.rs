use std::sync::Mutex;

fn free(m: &Mutex<Vec<i32>>) -> bool {
    m.try_lock().is_ok()
}

fn main() {
    let m = Mutex::new(vec![1, 2]);

    if m.lock().unwrap().len() == 2 {
        println!("in the if body: lock free is {}", free(&m));
    }

    match m.lock().unwrap().len() {
        2 => println!("in the match arm: lock free is {}", free(&m)),
        _ => {}
    }

    if let Some(first) = m.lock().unwrap().first().copied() {
        println!(
            "in the if let body (first is {first}): lock free is {}",
            free(&m)
        );
    }

    let n = m.lock().unwrap().len();
    match n {
        2 => println!("after `let n`: lock free is {}", free(&m)),
        _ => {}
    }

    while let Some(x) = m.lock().unwrap().pop() {
        println!("popped {x}: lock free is {}", free(&m));
    }

    if let Some(first) = m.lock().unwrap().first().copied() {
        println!("unreachable: {first}");
    } else {
        println!(
            "in the else branch (edition 2024): lock free is {}",
            free(&m)
        );
    }
}
