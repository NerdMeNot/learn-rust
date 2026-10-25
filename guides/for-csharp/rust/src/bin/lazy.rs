use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

async fn save() -> i32 {
    println!("saving");
    42
}

// A tiny executor, only to show when the body runs. It spins, so it suits only futures
// that finish at once. Real programs use a runtime (Tokio).
fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

fn main() {
    let fut = save();
    println!("called save");
    println!("{}", block_on(fut));
}
