use std::time::{Duration, Instant};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let start = Instant::now();
    let ticker = tokio::spawn(async move {
        for i in 1..=3 {
            // Rounded down to 50 ms, so small delays do not show.
            println!("tick {i} at {} ms", start.elapsed().as_millis() / 50 * 50);
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });
    // spawn_blocking runs the closure on a thread kept for blocking work.
    let report = tokio::task::spawn_blocking(|| {
        std::thread::sleep(Duration::from_millis(350));
        println!("report done");
    });
    ticker.await.unwrap();
    report.await.unwrap();
}
