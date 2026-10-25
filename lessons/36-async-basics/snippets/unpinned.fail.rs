use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
async fn f() -> u32 {
    1
}
fn main() {
    let mut fut = f();
    let mut cx = Context::from_waker(Waker::noop());
    let p = Pin::new(&mut fut).poll(&mut cx);
    println!("{:?}", p);
}
