use std::thread;

fn main() {
    let words = vec![String::from("alpha"), String::from("beta")];

    // Send: the Vec may move to another thread, and come back as its result
    let words = thread::spawn(move || words).join().unwrap();
    println!("moved there and back: {words:?}");

    // Sync: two threads may read the same Vec at once, through `&`
    let (a, b) = thread::scope(|s| {
        let first = s.spawn(|| words[0].len());
        let second = s.spawn(|| words[1].len());
        (first.join().unwrap(), second.join().unwrap())
    });
    println!("lengths read in parallel: {a} and {b}");
}
