use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Mutex, TryLockError};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};

type Task = Pin<Box<dyn Future<Output = ()>>>;

/// What the tasks write down, and how long after the start they wrote it.
#[derive(Clone)]
struct Log {
    start: Instant,
    entries: Rc<RefCell<Vec<(String, Duration)>>>,
}

impl Log {
    fn new() -> Log {
        Log {
            start: Instant::now(),
            entries: Rc::default(),
        }
    }
    fn push(&self, text: impl Into<String>) {
        self.entries
            .borrow_mut()
            .push((text.into(), self.start.elapsed()));
    }
    fn text(&self) -> String {
        let names: Vec<String> = self
            .entries
            .borrow()
            .iter()
            .map(|(t, _)| t.clone())
            .collect();
        names.join(" ")
    }
    fn when(&self, text: &str) -> Duration {
        self.entries
            .borrow()
            .iter()
            .find(|(t, _)| t == text)
            .unwrap()
            .1
    }
}

/// Pending once, then ready. `.await`ing it hands the thread back to the executor.
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

/// One thread, tasks polled in turn: the smallest executor.
fn run_all(mut tasks: Vec<Task>) {
    let mut cx = Context::from_waker(Waker::noop());
    while !tasks.is_empty() {
        let mut i = 0;
        while i < tasks.len() {
            match tasks[i].as_mut().poll(&mut cx) {
                Poll::Ready(()) => {
                    drop(tasks.remove(i)); // finished: drop the future
                }
                Poll::Pending => i += 1,
            }
        }
    }
}

/// Three steps, awaiting between them.
async fn polite(name: &'static str, log: Log) {
    for step in 1..=3 {
        log.push(format!("{name}{step}"));
        YieldNow(false).await;
    }
}

/// Three steps with no `.await` between them. The sleep stands in for a call that blocks,
/// such as a read from a slow disk.
async fn blocker(name: &'static str, log: Log) {
    for step in 1..=3 {
        log.push(format!("{name}{step}"));
        thread::sleep(Duration::from_millis(20));
    }
}

/// Takes a std lock and waits with the guard still alive.
async fn holder(lock: Rc<Mutex<u32>>, log: Log) {
    let _guard = lock.lock().unwrap();
    log.push("holder: has the lock, waits");
    YieldNow(false).await;
    log.push("holder: done");
}

/// `try_lock` reports what `lock` would have done: block this thread.
async fn asker(lock: Rc<Mutex<u32>>, log: Log) {
    match lock.try_lock() {
        Err(TryLockError::WouldBlock) => log.push("asker: lock() would block the only thread"),
        _ => log.push("asker: got the lock"),
    }
}

fn main() {
    let log = Log::new();
    run_all(vec![
        Box::pin(polite("a", log.clone())),
        Box::pin(polite("b", log.clone())),
    ]);
    println!("both await between steps: {}", log.text());

    let log = Log::new();
    run_all(vec![
        Box::pin(blocker("a", log.clone())),
        Box::pin(polite("b", log.clone())),
    ]);
    println!("a never awaits:           {}", log.text());
    println!(
        "b1 came after a's three 20 ms sleeps: {}",
        log.when("b1") >= Duration::from_millis(60)
    );

    let log = Log::new();
    let lock = Rc::new(Mutex::new(0));
    run_all(vec![
        Box::pin(holder(lock.clone(), log.clone())),
        Box::pin(asker(lock.clone(), log.clone())),
    ]);
    for (line, _) in log.entries.borrow().iter() {
        println!("{line}");
    }
}
