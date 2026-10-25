use std::future::{Future, ready};
use std::sync::Mutex;

fn spawn<F: Future + Send + 'static>(_future: F) {}

static LOG: Mutex<Vec<i32>> = Mutex::new(Vec::new());

fn main() {
    spawn(async {
        let mut log = LOG.lock().unwrap();
        log.push(1);
        drop(log); // dropped before the await, and still refused on 1.99.1
        ready(()).await;
        LOG.lock().unwrap().push(2);
    });
}
