use std::future::Future;
use std::mem::{size_of, size_of_val};
use std::pin::Pin;
use std::task::{Context, Poll};

/// Pending once, then ready: the smallest thing an `.await` can wait on.
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

async fn nothing_kept() -> usize {
    YieldNow(false).await;
    1
}

async fn array_kept() -> usize {
    let buf = [0u8; 1000];
    YieldNow(false).await;
    buf.len()
}

async fn array_finished_first() -> usize {
    let n = {
        let buf = [0u8; 1000];
        buf.len()
    };
    YieldNow(false).await;
    n
}

fn main() {
    println!(
        "the YieldNow it waits on:        {} byte",
        size_of::<YieldNow>()
    );
    println!(
        "nothing kept across the await: {} bytes",
        size_of_val(&nothing_kept())
    );
    println!(
        "[u8; 1000] kept across it:      {} bytes",
        size_of_val(&array_kept())
    );
    println!(
        "[u8; 1000] used up before it:   {} bytes",
        size_of_val(&array_finished_first())
    );
}
