use std::collections::{HashMap, VecDeque};

/// A job that needs `left` more units of work.
#[derive(Debug)]
pub struct Job {
    name: &'static str,
    left: u32,
}

/// Give the job at the front of the queue one turn of `quantum` units of work.
/// If it still has work left it goes to the back of the queue; otherwise it is finished
/// and leaves. Returns the name of the job that had the turn, or `None` for an empty queue.
pub fn serve(queue: &mut VecDeque<Job>, quantum: u32) -> Option<&'static str> {
    let mut job = queue.pop_front()?;
    job.left = job.left.saturating_sub(quantum);
    let name = job.name;
    if job.left > 0 {
        queue.push_back(job);
    }
    Some(name)
}

fn address(job: &Job) -> usize {
    job as *const Job as usize
}

fn main() {
    let job = |name, left| Job { name, left };

    // A small queue, turn by turn: a needs 3 units, b needs 1, c needs 2.
    let mut q = VecDeque::from([job("a", 3), job("b", 1), job("c", 2)]);
    let mut turns = Vec::new();
    while let Some(name) = serve(&mut q, 2) {
        turns.push(name);
        assert!(
            turns.len() < 100,
            "the queue never empties: a finished job must leave"
        );
    }
    assert_eq!(turns, ["a", "b", "c", "a"], "who gets each turn");
    assert!(q.is_empty());
    assert_eq!(serve(&mut q, 2), None, "an empty queue");

    // A long queue. Names come from a leaked list so they are 'static.
    let names: Vec<&'static str> = (0..300)
        .map(|i| &*Box::leak(format!("job{i}").into_boxed_str()))
        .collect();
    let work = |i: usize| 1 + (i as u32 * 7) % 5;
    let mut q = VecDeque::with_capacity(names.len() + 1);
    q.extend(names.iter().enumerate().map(|(i, &n)| job(n, work(i))));

    // The turns the plain rule gives: go round the jobs in order until all are done.
    let mut left: Vec<u32> = (0..names.len()).map(work).collect();
    let mut expected = Vec::new();
    while left.iter().any(|&l| l > 0) {
        for (i, l) in left.iter_mut().enumerate() {
            if *l > 0 {
                *l = l.saturating_sub(2);
                expected.push(names[i]);
            }
        }
    }

    // pop_front is O(1), so it cannot move the other jobs: each job's address may change
    // only when that job itself was just moved to the back.
    let mut at: HashMap<&str, usize> = q.iter().map(|j| (j.name, address(j))).collect();
    let mut turns = Vec::new();
    while let Some(name) = serve(&mut q, 2) {
        turns.push(name);
        assert!(
            turns.len() <= expected.len(),
            "more turns than the jobs need"
        );
        for j in q.iter() {
            let now = address(j);
            if j.name == name {
                at.insert(j.name, now);
            } else {
                assert_eq!(
                    at[j.name], now,
                    "{} moved in memory: take from one end and add to the other, do not rebuild the queue",
                    j.name
                );
            }
        }
    }
    assert_eq!(turns, expected, "who gets each turn");
    println!("all checks passed");
}
