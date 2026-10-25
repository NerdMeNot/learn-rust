use std::time::Duration;
use tokio::time::{sleep, timeout};

async fn fetch(name: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await; // pretend this is network work
    format!("{name} done after {ms}ms")
}

#[tokio::main]
async fn main() {
    // run two tasks at the same time, then wait for each
    let users = tokio::spawn(fetch("users", 30));
    let orders = tokio::spawn(fetch("orders", 10));
    println!("{}", users.await.unwrap());
    println!("{}", orders.await.unwrap());

    // join! runs both at once without spawning tasks
    let (left, right) = tokio::join!(fetch("left", 20), fetch("right", 20));
    println!("{left} / {right}");

    // give up after a deadline, like context.WithTimeout in Go
    match timeout(Duration::from_millis(50), fetch("slow", 500)).await {
        Ok(result) => println!("{result}"),
        Err(_) => println!("slow call timed out"),
    }
}
