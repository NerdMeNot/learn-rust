use std::sync::mpsc;
use std::thread;

fn main() {
    let (num_tx, num_rx) = mpsc::sync_channel(2);
    let (square_tx, square_rx) = mpsc::sync_channel(2);

    let generate = thread::spawn(move || {
        let mut sent = 0;
        for n in 1..=1_000_000u32 {
            if num_tx.send(n).is_err() {
                break; // stage 2 has quit
            }
            sent += 1;
        }
        sent
    });
    let square = thread::spawn(move || {
        for n in num_rx {
            if square_tx.send(n * n).is_err() {
                break; // main has quit
            }
        }
    });

    // main takes three results and hangs up
    let first: Vec<u32> = square_rx.iter().take(3).collect();
    drop(square_rx);

    square.join().unwrap();
    let sent = generate.join().unwrap();
    println!("main took {first:?} and hung up");
    println!("the generator stopped long before 1000000: {}", sent < 1000);
}
