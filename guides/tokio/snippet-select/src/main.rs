use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::time::sleep;

// Takes an item off the channel, then waits before handing it over.
async fn slow_next(rx: &mut mpsc::Receiver<u32>) -> Option<u32> {
    let item = rx.recv().await?;
    sleep(Duration::from_millis(20)).await;
    Some(item)
}

// A channel already holding 1, 2 and 3.
fn filled() -> mpsc::Receiver<u32> {
    let (tx, rx) = mpsc::channel(3);
    for n in 1..=3 {
        tx.try_send(n).unwrap();
    }
    rx
}

// A stop signal that is already sent.
fn stop_now() -> oneshot::Receiver<()> {
    let (stop, stopped) = oneshot::channel();
    stop.send(()).unwrap();
    stopped
}

#[tokio::main]
async fn main() {
    // `biased;` polls the branches in the order written, so the result is the same every run.
    let mut rx = filled();
    tokio::select! {
        biased;
        item = slow_next(&mut rx) => println!("slow_next gave {item:?}"),
        _ = stop_now() => println!("stop won, slow_next was dropped"),
    }
    let mut left = Vec::new();
    while let Ok(n) = rx.try_recv() {
        left.push(n);
    }
    println!("left in the channel: {left:?}");

    // The same race with `recv` itself, which Tokio documents as cancellation safe.
    let (tx, mut rx) = mpsc::channel::<u32>(1);
    tokio::select! {
        biased;
        item = rx.recv() => println!("recv gave {item:?}"),
        _ = stop_now() => println!("stop won, recv was dropped"),
    }
    tx.send(7).await.unwrap();
    println!("the next recv gives {:?}", rx.recv().await);
}
