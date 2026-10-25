use std::sync::{Arc, Mutex};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let hits = Arc::new(Mutex::new(0));
    let h = Arc::clone(&hits);
    tokio::spawn(async move {
        let mut n = h.lock().unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await; // still holding the lock
        *n += 1;
    })
    .await
    .unwrap();
    println!("{}", hits.lock().unwrap());
}
