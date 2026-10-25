use std::sync::Barrier;
use std::thread;

const N: usize = 10_000;

fn main() {
    // wait() returns only once N + 1 threads are waiting: N workers and main
    let barrier = Barrier::new(N + 1);

    let total: usize = thread::scope(|s| {
        let handles: Vec<_> = (0..N)
            .map(|_| {
                s.spawn(|| {
                    barrier.wait();
                    1
                })
            })
            .collect();
        barrier.wait(); // so all N threads were alive at the same time
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    });

    println!("threads alive at once: {N}, each added 1, total: {total}");
    assert_eq!(total, N);
}
