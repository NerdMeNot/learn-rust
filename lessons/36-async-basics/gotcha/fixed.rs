use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

async fn load_count() -> u32 {
    3
}

async fn report() -> u32 {
    let n: u32 = load_count().await;
    n + 1
}

/// A runtime in miniature: poll until the future is ready.
fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(value) = fut.as_mut().poll(&mut cx) {
            return value;
        }
    }
}

fn main() {
    println!("count: {}", block_on(report()));
}
