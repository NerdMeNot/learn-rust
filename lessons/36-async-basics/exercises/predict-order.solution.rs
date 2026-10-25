#![forbid(unsafe_code)]

use std::cell::RefCell;
use std::future::Future;
use std::pin::pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

type Log = Rc<RefCell<Vec<&'static str>>>;

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(value) = fut.as_mut().poll(&mut cx) {
            return value;
        }
    }
}

/// An async fn that records its name when its body runs.
async fn step(name: &'static str, log: Log) {
    log.borrow_mut().push(name);
}

/// Scenario 1: create one future, record "created", then run it.
fn create_then_run() -> Vec<&'static str> {
    let log: Log = Rc::default();
    let future = step("body", log.clone());
    log.borrow_mut().push("created");
    block_on(future);
    log.borrow().clone()
}

/// Scenario 2: create `a` and then `b`, but run `b` first.
fn run_in_reverse() -> Vec<&'static str> {
    let log: Log = Rc::default();
    let a = step("a", log.clone());
    let b = step("b", log.clone());
    block_on(b);
    block_on(a);
    log.borrow().clone()
}

/// Scenario 3: create a future, record "created", and never run it.
fn never_run() -> Vec<&'static str> {
    let log: Log = Rc::default();
    let future = step("body", log.clone());
    log.borrow_mut().push("created");
    drop(future);
    log.borrow().clone()
}

/// What each scenario's log holds at the end, in order. Write your prediction,
/// then let the checks in `main` run the real code.
fn predictions() -> [Vec<&'static str>; 3] {
    [vec!["created", "body"], vec!["b", "a"], vec!["created"]]
}

fn main() {
    let [first, second, third] = predictions();
    assert_eq!(first, create_then_run(), "scenario 1");
    assert_eq!(second, run_in_reverse(), "scenario 2");
    assert_eq!(third, never_run(), "scenario 3");
    println!("all checks passed");
}
