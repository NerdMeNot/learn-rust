use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

fn greeting(name: String) -> impl Future<Output = usize> {
    async move {
        println!("  the block runs now, with name = {name}");
        name.len()
    }
}

fn main() {
    let name = String::from("ferris");

    // an async block borrows what it uses, like a closure
    let borrowing = async { name.len() };
    println!("block built; name is still ours: {name}");
    println!("length: {}", block_on(borrowing));

    // `async move` takes ownership, so the future can outlive this scope
    let future = greeting(name);
    println!("future built; its body has not run");
    println!("length: {}", block_on(future));
}
