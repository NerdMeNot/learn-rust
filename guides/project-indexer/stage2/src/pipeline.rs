use std::{collections::BTreeMap, future::Future, path::PathBuf, sync::Arc};

use tokio::{sync::Semaphore, task::JoinSet};

/// Runs `work` on every path, with at most `limit` running at the same time.
pub async fn run<T, F, Fut>(paths: Vec<PathBuf>, limit: usize, work: F) -> BTreeMap<PathBuf, T>
where
    T: Send + 'static,
    F: Fn(PathBuf) -> Fut,
    Fut: Future<Output = T> + Send + 'static,
{
    let permits = Arc::new(Semaphore::new(limit));
    let mut tasks = JoinSet::new();
    for path in paths {
        // Wait here until one of the `limit` permits is free.
        let permit = permits.clone().acquire_owned().await.expect("the semaphore is never closed");
        let job = work(path.clone());
        tasks.spawn(async move {
            let _permit = permit; // released when this task ends
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

    // `start_paused` stops the clock: a sleep finishes the moment nothing else can run, so
    // this test takes no real time and gives the same answer every time.
    #[tokio::test(start_paused = true)]
    async fn never_more_than_limit_at_once() {
        let running = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let (r, p) = (running.clone(), peak.clone());
        let done = run(names(10), 3, move |_| {
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
    async fn results_are_sorted_whatever_order_the_work_finishes_in() {
        // The earlier the name, the longer the sleep, so the files finish in reverse order.
        let done = run(names(5), 5, |p: PathBuf| async move {
            let n = p.to_string_lossy()[1..].parse::<u64>().unwrap();
            tokio::time::sleep(Duration::from_millis(100 - n * 10)).await;
            n
        })
        .await;
        let order: Vec<u64> = done.values().copied().collect();
        assert_eq!(order, [0, 1, 2, 3, 4]);
    }
}
