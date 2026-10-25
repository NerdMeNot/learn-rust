use std::sync::{Arc, Mutex};

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

struct Job {
    id: u32,
    name: String,
    tags: Vec<String>,
}

fn main() {
    // each call compiles only if the bound holds; nothing happens when it runs
    assert_send::<i32>();
    assert_send::<Vec<String>>();
    assert_sync::<String>();
    assert_send::<Arc<Mutex<Vec<String>>>>();
    assert_sync::<Arc<Mutex<Vec<String>>>>();

    // no `impl Send for Job` anywhere: the compiler derives it from the fields
    assert_send::<Job>();
    assert_sync::<Job>();

    let job = Job {
        id: 7,
        name: String::from("resize"),
        tags: vec![String::from("image")],
    };
    println!("{} {} {:?}: Send and Sync", job.id, job.name, job.tags);
}
