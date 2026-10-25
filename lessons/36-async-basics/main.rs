use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

async fn task() {
    println!("task body runs");
}

fn main() {
    println!("start");
    let mut future = pin!(task());
    println!("task created");

    // a runtime does this for you: ask the future to run
    let mut cx = Context::from_waker(Waker::noop());
    let state = future.as_mut().poll(&mut cx);
    assert_eq!(state, Poll::Ready(()));
    println!("end");
}
