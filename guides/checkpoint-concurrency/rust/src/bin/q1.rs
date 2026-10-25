use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(value) = fut.as_mut().poll(&mut cx) {
            return value;
        }
    }
}

async fn step(name: &str) {
    println!("step {name}");
}

fn main() {
    let a = step("a");
    let b = step("b");
    println!("built both");
    block_on(b);
    block_on(a);
}
