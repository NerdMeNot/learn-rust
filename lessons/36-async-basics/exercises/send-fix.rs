#![forbid(unsafe_code)]

use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

/// Pending once, then ready: stands in for waiting on a file or a socket.
struct YieldNow(bool);

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            return Poll::Ready(());
        }
        self.0 = true;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

type Names = Arc<Mutex<Vec<String>>>;

/// Adds `name` to the shared list, waits (`YieldNow(false).await`), and returns how
/// long the list is AFTER the wait. The returned future must be `Send`, so a runtime
/// may move it to another thread while it waits. `main` checks that, and checks that
/// other tasks can use the list while this one waits.
async fn register(names: Names, name: &str) -> usize {
    todo!("push the name, wait, then read the length. Mind what is alive across the await")
}

/// Compile-time check: only accepts futures that are `Send`.
fn assert_send<T: Send>(_: &T) {}

fn main() {
    let names: Names = Arc::default();
    let mut cx = Context::from_waker(Waker::noop());

    let first = register(names.clone(), "ann");
    let second = register(names.clone(), "bob");
    assert_send(&first);
    assert_send(&second);
    let mut first = pin!(first);
    let mut second = pin!(second);

    // the first task adds its name, then waits
    assert_eq!(first.as_mut().poll(&mut cx), Poll::Pending);
    assert_eq!(*names.lock().unwrap(), ["ann"]);

    // while it waits, nobody may be holding the lock
    assert!(
        names.try_lock().is_ok(),
        "the lock is still held while the task waits"
    );

    // the second task gets in during that wait
    assert_eq!(second.as_mut().poll(&mut cx), Poll::Pending);
    assert_eq!(*names.lock().unwrap(), ["ann", "bob"]);

    // both finish and see both names: the length is read after the wait
    assert_eq!(first.as_mut().poll(&mut cx), Poll::Ready(2));
    assert_eq!(second.as_mut().poll(&mut cx), Poll::Ready(2));

    // a task that starts after the others are done sees all three names
    let third = register(names.clone(), "cy");
    assert_send(&third);
    let mut third = pin!(third);
    assert_eq!(third.as_mut().poll(&mut cx), Poll::Pending);
    assert_eq!(third.as_mut().poll(&mut cx), Poll::Ready(3));

    println!("all checks passed");
}
