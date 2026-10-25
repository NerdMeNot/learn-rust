use std::cell::RefCell;
use std::future::{Future, poll_fn};
use std::pin::{Pin, pin};
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

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

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

/// Runs both futures at once on this thread and returns both results.
async fn join_two<A: Future, B: Future>(a: A, b: B) -> (A::Output, B::Output) {
    let mut a = pin!(a);
    let mut b = pin!(b);
    let (mut a_out, mut b_out) = (None, None);
    poll_fn(|cx| {
        if a_out.is_none() {
            if let Poll::Ready(v) = a.as_mut().poll(cx) {
                a_out = Some(v);
            }
        }
        if b_out.is_none() {
            if let Poll::Ready(v) = b.as_mut().poll(cx) {
                b_out = Some(v);
            }
        }
        if a_out.is_some() && b_out.is_some() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    })
    .await;
    (a_out.unwrap(), b_out.unwrap())
}

async fn count(name: &'static str, n: u32, log: Rc<RefCell<Vec<String>>>) -> u32 {
    for i in 1..=n {
        log.borrow_mut().push(format!("{name}{i}"));
        YieldNow(false).await;
    }
    n * 10
}

fn main() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let (a, b) = block_on(async {
        let a = count("a", 3, log.clone()).await;
        let b = count("b", 2, log.clone()).await;
        (a, b)
    });
    println!(
        "one after the other: {a} {b}  steps: {}",
        log.borrow().join(" ")
    );

    log.borrow_mut().clear();
    let (a, b) = block_on(join_two(
        count("a", 3, log.clone()),
        count("b", 2, log.clone()),
    ));
    println!(
        "joined:              {a} {b}  steps: {}",
        log.borrow().join(" ")
    );
}
