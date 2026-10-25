use std::sync::mpsc;
use std::thread;

struct Noisy;
impl Drop for Noisy {
    fn drop(&mut self) {
        println!("child: Noisy dropped");
    }
}

fn main() {
    let (started_tx, started_rx) = mpsc::channel();

    // the handle is dropped at once: dropping a JoinHandle detaches the thread
    thread::spawn(move || {
        let _guard = Noisy;
        started_tx.send(()).unwrap();
        loop {
            thread::park(); // sleeps until unparked; nobody ever unparks it
        }
    });

    // the channel is only here to know the child is running before main ends
    started_rx.recv().unwrap();
    println!("main: the child is running, and main returns now");
}
