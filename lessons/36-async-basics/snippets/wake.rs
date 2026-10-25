use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll, Wake, Waker};

/// A waker that remembers it was woken.
struct Flag(AtomicBool);

impl Wake for Flag {
    fn wake(self: Arc<Self>) {
        self.0.store(true, Ordering::SeqCst);
    }
}

/// Pending `pending` times, then ready. Tells the waker only if `wake` is true.
struct Slow {
    pending: u32,
    wake: bool,
}

impl Future for Slow {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.pending == 0 {
            return Poll::Ready("ready");
        }
        self.pending -= 1;
        if self.wake {
            cx.waker().wake_by_ref();
        }
        Poll::Pending
    }
}

/// Polls again only when the future asked to be. Gives up instead of waiting forever.
fn run(fut: impl Future<Output = &'static str>) -> Option<&'static str> {
    let flag = Arc::new(Flag(AtomicBool::new(false)));
    let waker = Waker::from(flag.clone());
    let mut cx = Context::from_waker(&waker);
    let mut fut = pin!(fut);
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return Some(v);
        }
        if !flag.0.swap(false, Ordering::SeqCst) {
            return None; // a real executor would sleep here, with nothing to wake it
        }
    }
}

fn main() {
    println!(
        "wakes before each Pending: {:?}",
        run(Slow {
            pending: 3,
            wake: true
        })
    );
    println!(
        "never wakes: {:?}",
        run(Slow {
            pending: 3,
            wake: false
        })
    );
}
