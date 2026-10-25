use std::sync::{Arc, Mutex};
use std::thread;

struct Job {
    name: String,
    log: Arc<Mutex<Vec<String>>>,
}

fn run(job: Job) {
    job.log.lock().unwrap().push(job.name);
}

fn main() {
    let log = Arc::new(Mutex::new(Vec::new()));
    let job = Job {
        name: String::from("resize"),
        log: Arc::clone(&log),
    };
    thread::spawn(move || run(job)).join().unwrap();
    println!("log: {:?}", log.lock().unwrap());
}
