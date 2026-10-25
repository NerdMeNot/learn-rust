use std::collections::HashMap;
use std::sync::Mutex;

fn main() {
    // One lock around the whole Vec: holding it for element 0 also shuts out element 1.
    let whole = Mutex::new(vec![0, 0]);
    let guard = whole.lock().unwrap();
    assert!(whole.try_lock().is_err());
    drop(guard);

    // One lock per element: element 1 stays available.
    let each = vec![Mutex::new(0), Mutex::new(0)];
    let guard0 = each[0].lock().unwrap();
    assert!(each[0].try_lock().is_err());
    assert!(each[1].try_lock().is_ok());
    // A view of both elements holds both locks.
    let guard1 = each[1].lock().unwrap();
    assert!(each[0].try_lock().is_err() && each[1].try_lock().is_err());
    drop((guard0, guard1));

    // A map split into shards by key: other shards stay available, the same shard does not.
    let shards: Vec<Mutex<HashMap<u32, u32>>> =
        (0..4).map(|_| Mutex::new(HashMap::new())).collect();
    let shard = |key: u32| &shards[key as usize % 4];
    let guard = shard(1).lock().unwrap();
    assert!(shard(2).try_lock().is_ok());
    assert!(shard(5).try_lock().is_err());
    drop(guard);

    println!("all granularity assertions passed");
}
