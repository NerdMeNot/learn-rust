use std::sync::{Arc, Mutex};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let hits = Arc::new(Mutex::new(0));
    let h = Arc::clone(&hits);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(10)).await; // wait first
        *h.lock().unwrap() += 1; // then lock, change, unlock, all on one line
    })
    .await
    .unwrap();
    println!("{}", hits.lock().unwrap());
}
