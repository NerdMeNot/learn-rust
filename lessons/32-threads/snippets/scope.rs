use std::thread;

fn main() {
    let words = vec!["alpha", "beta", "gamma", "delta"];
    let mut longest = 0;

    // no move, no Arc: the scope joins every thread before it returns
    let total = thread::scope(|s| {
        let counting = s.spawn(|| words.iter().map(|w| w.len()).sum::<usize>());
        s.spawn(|| longest = words.iter().map(|w| w.len()).max().unwrap_or(0));
        counting.join().unwrap()
    });

    println!("total letters: {total}");
    println!("longest word: {longest}");
    println!("words is still ours: {}", words.len());
}
