use std::time::Duration;

#[tokio::main]
async fn main() {
    tokio::time::delay_for(Duration::from_millis(100)).await;
    println!("waited");
}
