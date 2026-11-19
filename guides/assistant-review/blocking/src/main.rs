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
    let report = tokio::spawn(async {
        // Stands in for slow work that blocks: a big file read, a password hash.
        std::thread::sleep(Duration::from_millis(350));
        println!("report done");
    });
    ticker.await.unwrap();
    report.await.unwrap();
}
