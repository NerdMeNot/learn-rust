use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // The program from the gotcha, with Tokio's Mutex: this one compiles, even with `spawn`.
    let hits = Arc::new(Mutex::new(0));
    let h = Arc::clone(&hits);
    tokio::spawn(async move {
        let mut n = h.lock().await;
        sleep(Duration::from_millis(10)).await; // holding the lock across an await is what it is for
        *n += 1;
    })
    .await
    .unwrap();
    println!("hits: {}", hits.lock().await);

    // The holder/asker from the previous run: the asker waits its turn and nothing hangs.
    let lock = Mutex::new(0);
    let holder = async {
        let mut n = lock.lock().await;
        println!("holder: has the lock, now waits");
        sleep(Duration::from_millis(10)).await;
        *n += 1;
        println!("holder: done");
    };
    let asker = async {
        println!("asker: asks for the lock");
        let n = lock.lock().await;
        println!("asker: got it, value {}", *n);
    };
    tokio::join!(holder, asker);
}
