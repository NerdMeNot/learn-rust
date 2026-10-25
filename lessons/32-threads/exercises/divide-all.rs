#![forbid(unsafe_code)]

use std::sync::Mutex;
use std::thread::{self, ThreadId};

// The checks use this to record which thread ran each call. Ignore it for now.
static SEEN: Mutex<Vec<ThreadId>> = Mutex::new(Vec::new());

/// The work for one input. It records which thread ran it. It panics on 0 (with fixed text)
/// and on 7 (with a message built from the input).
fn work(n: i32) -> i32 {
    SEEN.lock().unwrap().push(thread::current().id());
    assert!(n != 7, "unlucky input {n}");
    100 / n
}

/// Calls `work` for every input, each call on its own new thread. Gives back one entry per
/// input, in the order of `inputs`: `Ok(result)`, or `Err(message)` if that call panicked.
fn run_all(inputs: Vec<i32>) -> Vec<Result<i32, String>> {
    todo!(
        "keep the JoinHandles, then join them in order; a failed join holds the panic's payload, which you can ask for a &str or a String (see 'A thread that panics')"
    )
}

fn main() {
    let got = run_all(vec![5, 0, 7, 20]);
    assert_eq!(
        got,
        vec![
            Ok(20),
            Err("attempt to divide by zero".to_string()),
            Err("unlucky input 7".to_string()),
            Ok(5)
        ],
        "one entry per input, in input order; a panic must not stop the others"
    );

    // every call ran on a thread of its own, and not on main's
    {
        let seen = SEEN.lock().unwrap();
        assert_eq!(seen.len(), 4);
        let mut ids: Vec<_> = seen.iter().map(|id| format!("{id:?}")).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 4, "each input needs its own thread");
        assert!(!seen.contains(&thread::current().id()), "not on main");
    }

    assert_eq!(run_all(vec![]), Vec::<Result<i32, String>>::new());
    assert_eq!(run_all(vec![4]), vec![Ok(25)]);

    println!("all checks passed");
}
