use std::{collections::BTreeMap, future::Future, path::PathBuf, sync::Arc};

use tokio::{
    sync::{Semaphore, mpsc},
    task::JoinSet,
};
use tokio_util::sync::CancellationToken;

/// What `run` hands back: the finished work, and whether it was stopped early.
pub struct Report<T> {
    pub done: BTreeMap<PathBuf, T>,
    pub cancelled: bool,
}

/// Takes paths from `rx` and runs `work` on them, at most `limit` at a time, until the
/// paths run out or `cancel` is triggered.
pub async fn run<T, F, Fut>(
    mut rx: mpsc::Receiver<PathBuf>,
    limit: usize,
    cancel: CancellationToken,
    work: F,
) -> Report<T>
where
    T: Send + 'static,
    F: Fn(PathBuf) -> Fut,
    Fut: Future<Output = T> + Send + 'static,
{
    let permits = Arc::new(Semaphore::new(limit));
    let mut tasks = JoinSet::new();
    loop {
        // Permit first, path second. While every worker is busy this loop does not call
        // `recv`, the channel fills up, and the walker waits in `send`.
        let permit = tokio::select! {
            biased;
            _ = cancel.cancelled() => break,
            permit = permits.clone().acquire_owned() => permit.expect("the semaphore is never closed"),
        };
        let path = tokio::select! {
            biased;
            _ = cancel.cancelled() => break,
            next = rx.recv() => match next {
                Some(path) => path,
                None => break,
            },
        };
        let job = work(path.clone());
        let token = cancel.clone();
        tasks.spawn(async move {
            let _permit = permit;
            // Whichever finishes first wins; the loser is dropped, which cancels it.
            tokio::select! {
                biased;
                _ = token.cancelled() => None,
                value = job => Some((path, value)),
            }
        });
    }

    let mut done = BTreeMap::new();
    while let Some(joined) = tasks.join_next().await {
        if let Some((path, value)) = joined.expect("a worker panicked") {
            done.insert(path, value);
        }
    }
    Report { done, cancelled: cancel.is_cancelled() }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::atomic::{AtomicUsize, Ordering::SeqCst},
        time::Duration,
    };

    use super::*;

    fn names(n: usize) -> Vec<PathBuf> {
        (0..n).map(|i| PathBuf::from(format!("f{i:02}"))).collect()
    }

    /// A channel of `capacity` that a spawned task fills with `n` names.
    fn feed(n: usize, capacity: usize) -> mpsc::Receiver<PathBuf> {
        let (tx, rx) = mpsc::channel(capacity);
        tokio::spawn(async move {
            for path in names(n) {
                if tx.send(path).await.is_err() {
                    break;
                }
            }
        });
        rx
    }

    #[tokio::test(start_paused = true)]
    async fn never_more_than_limit_at_once() {
        let running = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let (r, p) = (running.clone(), peak.clone());
        let report = run(feed(10, 2), 3, CancellationToken::new(), move |_| {
            let (r, p) = (r.clone(), p.clone());
            async move {
                let now = r.fetch_add(1, SeqCst) + 1;
                p.fetch_max(now, SeqCst);
                tokio::time::sleep(Duration::from_millis(10)).await;
                r.fetch_sub(1, SeqCst);
            }
        })
        .await;
        assert_eq!(report.done.len(), 10);
        assert_eq!(peak.load(SeqCst), 3);
    }

    #[tokio::test(start_paused = true)]
    async fn the_producer_cannot_get_far_ahead_of_the_workers() {
        let capacity = 2;
        let (tx, rx) = mpsc::channel(capacity);
        let sent = Arc::new(AtomicUsize::new(0));
        let started = Arc::new(AtomicUsize::new(0));
        let worst = Arc::new(AtomicUsize::new(0));

        let s = sent.clone();
        tokio::spawn(async move {
            for path in names(10) {
                tx.send(path).await.unwrap();
                s.fetch_add(1, SeqCst);
            }
        });

        let (s, st, w) = (sent.clone(), started.clone(), worst.clone());
        // One worker, so the work is the slow part. At the start of each file, how many
        // paths has the producer sent that no worker has started yet?
        let report = run(rx, 1, CancellationToken::new(), move |_| {
            let (s, st, w) = (s.clone(), st.clone(), w.clone());
            async move {
                let started_now = st.fetch_add(1, SeqCst) + 1;
                w.fetch_max(s.load(SeqCst).saturating_sub(started_now), SeqCst);
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await;

        assert_eq!(report.done.len(), 10);
        assert_eq!(sent.load(SeqCst), 10);
        assert!(worst.load(SeqCst) <= capacity, "the producer got {} ahead", worst.load(SeqCst));
    }

    #[tokio::test(start_paused = true)]
    async fn cancelling_keeps_finished_work_and_drops_the_rest() {
        let cancel = CancellationToken::new();
        let trigger = cancel.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(25)).await;
            trigger.cancel();
        });
        // Two at a time, 10 ms each: f00 and f01 finish at 10 ms, f02 and f03 at 20 ms,
        // and f04 and f05 are still running when the cancel arrives at 25 ms.
        let report = run(feed(6, 2), 2, cancel, |_| async {
            tokio::time::sleep(Duration::from_millis(10)).await;
        })
        .await;
        assert!(report.cancelled);
        let finished: Vec<&str> = report.done.keys().map(|p| p.to_str().unwrap()).collect();
        assert_eq!(finished, ["f00", "f01", "f02", "f03"]);
    }

    #[tokio::test(start_paused = true)]
    async fn a_run_nobody_cancels_reports_not_cancelled() {
        let report = run(feed(3, 1), 2, CancellationToken::new(), |_| async {}).await;
        assert!(!report.cancelled);
        assert_eq!(report.done.len(), 3);
    }
}
