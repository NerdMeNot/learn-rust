use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

struct Job {
    name: String,
    log: Rc<RefCell<Vec<String>>>,
}

fn run(job: Job) {
    job.log.borrow_mut().push(job.name);
}

fn main() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let job = Job {
        name: String::from("resize"),
        log: Rc::clone(&log),
    };
    thread::spawn(move || run(job)).join().unwrap();
    println!("log: {:?}", log.borrow());
}
