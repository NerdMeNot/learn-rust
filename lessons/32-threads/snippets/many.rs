use std::thread;

fn main() {
    let inputs = vec![5u64, 3, 8, 1];

    // start every thread first, then join them in the order they were started
    let handles: Vec<_> = inputs
        .iter()
        .map(|&n| thread::spawn(move || (1..=n).product::<u64>()))
        .collect();

    let results: Vec<u64> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("factorials: {:?}", results);
    assert_eq!(results, vec![120, 6, 40320, 1]);
}
