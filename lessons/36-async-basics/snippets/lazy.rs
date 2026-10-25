use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

/// Polls `fut` until it is ready. Returns the output and the number of polls it took.
fn block_on<F: Future>(fut: F) -> (F::Output, u32) {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    let mut polls = 0;
    loop {
        polls += 1;
        if let Poll::Ready(value) = fut.as_mut().poll(&mut cx) {
            return (value, polls);
        }
    }
}

async fn fetch_name() -> String {
    println!("  fetch_name is running");
    String::from("ferris")
}

fn main() {
    println!("calling fetch_name()");
    let future = fetch_name();
    println!("called it, and nothing ran");

    let (name, polls) = block_on(future);
    println!("name: {name}, polls: {polls}");
}
