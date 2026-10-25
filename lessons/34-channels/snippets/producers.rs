use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let mut workers = Vec::new();
    for id in 1..=3 {
        let tx = tx.clone(); // one more sender, same channel
        workers.push(thread::spawn(move || {
            for step in 1..=3 {
                tx.send((id, step)).unwrap();
            }
        }));
    }
    drop(tx); // the original sender; without this the loop below would never end

    let mut all = Vec::new();
    let mut last_step = [0; 4];
    for (id, step) in rx {
        // messages from ONE sender arrive in the order it sent them
        assert_eq!(step, last_step[id] + 1);
        last_step[id] = step;
        all.push((id, step));
    }
    for w in workers {
        w.join().unwrap();
    }

    // the interleaving of different senders changes from run to run: sort before printing
    all.sort();
    println!("received {} messages", all.len());
    println!("sorted: {all:?}");
}
