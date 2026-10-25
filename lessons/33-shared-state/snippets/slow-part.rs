use std::sync::Mutex;

// Stands in for a slow call: a file read, a network request, a long calculation.
fn slow_part(results: &Mutex<Vec<u32>>) -> u32 {
    println!(
        "  slow part runs; lock is held by someone: {}",
        results.try_lock().is_err()
    );
    42
}

fn main() {
    let results = Mutex::new(Vec::new());

    println!("guard first, then the slow part:");
    {
        let mut guard = results.lock().unwrap();
        guard.push(slow_part(&results));
    }

    println!("slow part first, then a short lock:");
    let value = slow_part(&results);
    results.lock().unwrap().push(value);

    println!("results: {:?}", results.lock().unwrap());
}
