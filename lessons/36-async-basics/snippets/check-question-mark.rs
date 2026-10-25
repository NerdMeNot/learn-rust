use std::future::Future;
use std::num::ParseIntError;
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

async fn double(text: &str) -> Result<i32, ParseIntError> {
    let n: i32 = text.parse()?;
    Ok(n * 2)
}

fn main() {
    assert_eq!(block_on(double("21")), Ok(42));
    assert!(block_on(double("x")).is_err());
}
