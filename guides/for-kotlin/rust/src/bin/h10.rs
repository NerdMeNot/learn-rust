use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

async fn fetch(id: u32) -> String {
    println!("fetch {id} starts");
    format!("user{id}")
}

// std has the Future trait but no runtime; this is the smallest executor.
fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    // these futures never wait, so the first poll returns Ready
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

fn main() {
    let first = fetch(1);
    println!("after calling fetch");
    println!("{}", block_on(first));
    let _second = fetch(2);
    println!("end of main");
}
