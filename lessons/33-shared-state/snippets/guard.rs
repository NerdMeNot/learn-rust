use std::sync::Mutex;

fn main() {
    let m = Mutex::new(vec![1, 2, 3]);

    {
        let mut guard = m.lock().unwrap();
        guard.push(4);
        println!("guard alive: try_lock fails: {}", m.try_lock().is_err());
    }
    println!("block ended: try_lock works: {}", m.try_lock().is_ok());

    m.lock().unwrap().push(5);
    println!("statement ended: try_lock works: {}", m.try_lock().is_ok());

    let guard = m.lock().unwrap();
    println!("named guard: try_lock works: {}", m.try_lock().is_ok());
    drop(guard);
    println!(
        "after drop(guard): try_lock works: {}",
        m.try_lock().is_ok()
    );

    println!("data: {:?}", m.lock().unwrap());
}
