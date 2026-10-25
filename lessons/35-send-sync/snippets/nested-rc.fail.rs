use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

struct Log {
    lines: Rc<RefCell<Vec<String>>>,
}

struct Job {
    name: String,
    log: Log,
}

fn run(job: Job) {
    job.log.lines.borrow_mut().push(job.name);
}

fn main() {
    let job = Job {
        name: String::from("resize"),
        log: Log {
            lines: Rc::new(RefCell::new(Vec::new())),
        },
    };
    thread::spawn(move || run(job)).join().unwrap();
}
