use std::{collections::BTreeMap, future::Future, path::PathBuf, sync::Arc};

use tokio::{
    sync::{Semaphore, mpsc},
    task::JoinSet,
};

/// Takes paths from `rx` and runs `work` on them, at most `limit` at a time.
pub async fn run<T, F, Fut>(mut rx: mpsc::Receiver<PathBuf>, limit: usize, work: F) -> BTreeMap<PathBuf, T>
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
        let permit = permits.clone().acquire_owned().await.expect("the semaphore is never closed");
        let Some(path) = rx.recv().await else { break };
        let job = work(path.clone());
        tasks.spawn(async move {
            let _permit = permit;
            (path, job.await)
        });
    }

    let mut done = BTreeMap::new();
    while let Some(joined) = tasks.join_next().await {
        let (path, value) = joined.expect("a worker panicked");
        done.insert(path, value);
    }
    done
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

    #[tokio::test(start_paused = true)]
    async fn never_more_than_limit_at_once() {
        let (tx, rx) = mpsc::channel(2);
        tokio::spawn(async move {
            for path in names(10) {
                tx.send(path).await.unwrap();
            }
        });
        let running = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let (r, p) = (running.clone(), peak.clone());
        let done = run(rx, 3, move |_| {
            let (r, p) = (r.clone(), p.clone());
            async move {
                let now = r.fetch_add(1, SeqCst) + 1;
                p.fetch_max(now, SeqCst);
                tokio::time::sleep(Duration::from_millis(10)).await;
                r.fetch_sub(1, SeqCst);
            }
        })
        .await;
        assert_eq!(done.len(), 10);
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
        let done = run(rx, 1, move |_| {
            let (s, st, w) = (s.clone(), st.clone(), w.clone());
            async move {
                let started_now = st.fetch_add(1, SeqCst) + 1;
                w.fetch_max(s.load(SeqCst).saturating_sub(started_now), SeqCst);
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await;

        assert_eq!(done.len(), 10);
        assert_eq!(sent.load(SeqCst), 10);
        assert!(worst.load(SeqCst) <= capacity, "the producer got {} ahead", worst.load(SeqCst));
    }
}
