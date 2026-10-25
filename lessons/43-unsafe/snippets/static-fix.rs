use std::sync::Mutex;

// `Mutex::new` is a `const fn`, so it can start a `static`
static LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn main() {
    LOG.lock().unwrap().push(String::from("started"));
    println!("log: {:?}", LOG.lock().unwrap());
}
