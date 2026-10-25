use std::future::{Future, ready};
use std::pin::{Pin, pin};
use std::task::{Context, Poll, Waker};

/// Not ready the first time it is polled, ready the second time.
struct YieldNow {
    yielded: bool,
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            cx.waker().wake_by_ref(); // "poll me again soon"
            Poll::Pending
        }
    }
}

/// Polls `fut` until it is ready. Returns the output and the number of polls it took.
fn block_on<F: Future>(fut: F) -> (F::Output, u32) {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    let mut polls = 0;
    loop {
        polls += 1;
        if let Poll::Ready(value) = fut.as_mut().poll(&mut cx) {
            return (value, polls);
        }
    }
}

async fn wait_then_answer() -> u32 {
    println!("  before the await");
    YieldNow { yielded: false }.await;
    println!("  after the await");
    42
}

fn main() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut fut = pin!(YieldNow { yielded: false });
    println!("poll 1: {:?}", fut.as_mut().poll(&mut cx));
    println!("poll 2: {:?}", fut.as_mut().poll(&mut cx));

    let (answer, polls) = block_on(wait_then_answer());
    println!("answer: {answer}, polls: {polls}");

    // awaiting a future that is already ready does not stop the caller
    let ((), polls) = block_on(async { ready(()).await });
    println!("await on a ready future, polls: {polls}");
}
