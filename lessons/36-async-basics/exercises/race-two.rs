#![forbid(unsafe_code)]

use std::cell::RefCell;
use std::future::{Future, poll_fn};
use std::pin::{Pin, pin};
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

type Log = Rc<RefCell<Vec<String>>>;

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(value) = fut.as_mut().poll(&mut cx) {
            return value;
        }
    }
}

/// Polls `a` and `b` in turn on this thread, `a` first, and returns the output of
/// whichever is ready first. The other future is dropped, and never polled again.
async fn race_two<A, B>(a: A, b: B) -> A::Output
where
    A: Future,
    B: Future<Output = A::Output>,
{
    todo!(
        "pin both futures, then poll_fn: poll a; if it is not ready, poll b; return the first Ready"
    )
}

/// Pending `steps` times, recording "name1", "name2", ..., then ready with `answer`.
/// Records "drop name" when it is dropped. Panics if it is polled after it has finished.
struct Steps {
    name: &'static str,
    steps: u32,
    done: u32,
    answer: u32,
    finished: bool,
    log: Log,
}

fn steps(name: &'static str, steps: u32, answer: u32, log: &Log) -> Steps {
    Steps {
        name,
        steps,
        done: 0,
        answer,
        finished: false,
        log: log.clone(),
    }
}

impl Future for Steps {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        assert!(!self.finished, "{} was polled after it finished", self.name);
        if self.done == self.steps {
            self.finished = true;
            return Poll::Ready(self.answer);
        }
        self.done += 1;
        let entry = format!("{}{}", self.name, self.done);
        self.log.borrow_mut().push(entry);
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

impl Drop for Steps {
    fn drop(&mut self) {
        let entry = format!("drop {}", self.name);
        self.log.borrow_mut().push(entry);
    }
}

fn main() {
    let log: Log = Rc::default();

    // a finishes first: its answer wins, and b is dropped half-way
    let winner = block_on(race_two(steps("a", 2, 7, &log), steps("b", 3, 8, &log)));
    assert_eq!(winner, 7);
    let text = log.borrow().join(" ");
    // they took turns, a first, and nobody was polled after the race was decided
    assert!(text.starts_with("a1 b1 a2 b2 "), "turns: {text}");
    assert!(!text.contains("b3"), "b ran on after a had won: {text}");
    // by the time race_two has returned, the loser has been dropped (cancelled)
    assert_eq!(
        text.matches("drop b").count(),
        1,
        "the loser must be dropped: {text}"
    );

    // b finishes first
    log.borrow_mut().clear();
    let winner = block_on(race_two(steps("a", 3, 7, &log), steps("b", 1, 8, &log)));
    assert_eq!(winner, 8);
    let text = log.borrow().join(" ");
    assert!(text.starts_with("a1 b1 a2 "), "turns: {text}");
    assert!(!text.contains("a3"), "a ran on after b had won: {text}");
    assert_eq!(
        text.matches("drop a").count(),
        1,
        "the loser must be dropped: {text}"
    );

    // both are ready at once: a is polled first, so a wins and b is never polled
    log.borrow_mut().clear();
    let winner = block_on(race_two(steps("a", 0, 7, &log), steps("b", 0, 8, &log)));
    assert_eq!(winner, 7);
    let text = log.borrow().join(" ");
    assert!(!text.contains("b1"), "b was polled: {text}");
    assert_eq!(
        text.matches("drop b").count(),
        1,
        "the loser must be dropped: {text}"
    );

    println!("all checks passed");
}
