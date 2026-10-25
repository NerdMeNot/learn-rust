use std::sync::mpsc;
use std::thread;

fn main() {
    // stage 1 -> stage 2 -> main, each link a bounded channel
    let (num_tx, num_rx) = mpsc::sync_channel(2);
    let (square_tx, square_rx) = mpsc::sync_channel(2);

    let generate = thread::spawn(move || {
        for n in 1..=10u32 {
            num_tx.send(n).unwrap();
        }
        // num_tx is dropped here, so stage 2's loop will end
    });
    let square = thread::spawn(move || {
        for n in num_rx {
            square_tx.send(n * n).unwrap();
        }
        // square_tx is dropped here, so main's loop will end
    });

    let squares: Vec<u32> = square_rx.iter().collect();
    generate.join().unwrap();
    square.join().unwrap();
    println!("squares: {squares:?}");
    println!("sum: {}", squares.iter().sum::<u32>());
}
