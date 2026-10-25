use std::sync::Mutex;
use std::time::Duration;
use tokio::time::sleep;

// One thread runs everything, so there is no `spawn` and no `Send` check.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let lock = Mutex::new(0);

    let holder = async {
        let mut n = lock.lock().unwrap();
        println!("holder: has the lock, now waits");
        sleep(Duration::from_millis(10)).await;
        *n += 1;
        println!("holder: done");
    };
    let asker = async {
        println!("asker: asks for the lock");
        let n = lock.lock().unwrap(); // blocks the only thread
        println!("asker: got it, value {}", *n);
    };

    tokio::join!(holder, asker);
}
