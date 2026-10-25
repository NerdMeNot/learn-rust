use std::future::{Future, ready};
use std::sync::Mutex;

fn spawn<F: Future + Send + 'static>(_future: F) {}

static LOG: Mutex<Vec<i32>> = Mutex::new(Vec::new());

fn main() {
    spawn(async {
        LOG.lock().unwrap().push(1); // a temporary: gone at the semicolon
        {
            let mut log = LOG.lock().unwrap();
            log.push(2);
        } // a named guard in a block that ends before the await
        ready(()).await;
        LOG.lock().unwrap().push(3);
    });
    println!("accepted: no guard is alive at the await");
}
