use std::sync::{Arc, Mutex};
use std::thread;

struct Log {
    lines: Arc<Mutex<Vec<String>>>,
}

struct Job {
    name: String,
    log: Log,
}

fn run(job: Job) {
    job.log.lines.lock().unwrap().push(job.name);
}

fn main() {
    let lines = Arc::new(Mutex::new(Vec::new()));
    let job = Job {
        name: String::from("resize"),
        log: Log {
            lines: Arc::clone(&lines),
        },
    };
    thread::spawn(move || run(job)).join().unwrap();
    println!("lines: {:?}", lines.lock().unwrap());
}
