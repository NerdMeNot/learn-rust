use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::{Pin, pin};
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

struct Item(u32);

impl Drop for Item {
    fn drop(&mut self) {
        println!("  dropped item {}", self.0);
    }
}

type Queue = RefCell<VecDeque<Item>>;

/// Takes one item off the queue, waits, then delivers it.
async fn forward(queue: &Queue) {
    let item = queue.borrow_mut().pop_front().unwrap();
    YieldNow(false).await; // a drop can land here
    println!("  delivered item {}", item.0);
}

fn queue_of_three() -> Queue {
    RefCell::new((1..=3).map(Item).collect())
}

fn main() {
    let mut cx = Context::from_waker(Waker::noop());

    println!("polled until ready:");
    let queue = queue_of_three();
    let mut fut = pin!(forward(&queue));
    while fut.as_mut().poll(&mut cx).is_pending() {}
    println!("  left in the queue: {}", queue.borrow().len());

    println!("polled once, then dropped:");
    let queue = queue_of_three();
    {
        let mut fut = pin!(forward(&queue));
        let _ = fut.as_mut().poll(&mut cx);
        println!("  dropping the future now");
    }
    println!("  left in the queue: {}", queue.borrow().len());
    println!("end of main: both queues drop what is left");
}
