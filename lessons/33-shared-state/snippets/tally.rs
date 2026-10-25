use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let hits: Arc<Mutex<HashMap<&str, u32>>> = Arc::new(Mutex::new(HashMap::new()));

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let hits = Arc::clone(&hits);
            thread::spawn(move || {
                for page in ["home", "docs", "home"] {
                    // One lock, and the update happens inside it: Java's map.merge(page, 1, Integer::sum).
                    *hits.lock().unwrap().entry(page).or_insert(0) += 1;
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }

    let mut rows: Vec<_> = hits.lock().unwrap().iter().map(|(k, v)| (*k, *v)).collect();
    rows.sort();
    println!("hits: {rows:?}");
}
