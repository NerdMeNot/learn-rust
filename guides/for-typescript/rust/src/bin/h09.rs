use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

async fn fetch_name() -> String {
    println!("fetching");
    "ana".to_string()
}

// A runtime's job in miniature: poll once. This future never waits, so one poll is enough.
fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("this future waits; use a real runtime"),
    }
}

fn main() {
    let p = fetch_name(); // builds a future; the body has not started
    println!("created");
    println!("{}", block_on(p));
}
