use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Stands in for a database write.
async fn save(_count: u32) {
    tokio::time::sleep(Duration::from_millis(10)).await;
}

async fn record_visit(visits: Arc<Mutex<u32>>) {
    let count = {
        let mut count = visits.lock().unwrap();
        *count += 1;
        *count
    }; // the guard is dropped here, before the await
    save(count).await;
}

#[tokio::main]
async fn main() {
    let visits = Arc::new(Mutex::new(0));
    let mut tasks = Vec::new();
    for _ in 0..3 {
        tasks.push(tokio::spawn(record_visit(Arc::clone(&visits))));
    }
    for task in tasks {
        task.await.unwrap();
    }
    println!("total visits: {}", visits.lock().unwrap());
}
