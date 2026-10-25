use std::future::{Future, ready};
use std::pin::pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};
use std::thread;

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(value) = fut.as_mut().poll(&mut cx) {
            return value;
        }
    }
}

/// Like a runtime's `spawn`: the future may move to another thread, so it must be `Send`.
fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    thread::spawn(move || block_on(fut)).join().unwrap();
}

async fn count_words() {
    let n = {
        let words = Rc::new(vec!["one", "two"]);
        words.len()
    }; // words is gone before the await
    ready(()).await;
    println!("words: {n}");
}

fn main() {
    spawn(count_words());
}
