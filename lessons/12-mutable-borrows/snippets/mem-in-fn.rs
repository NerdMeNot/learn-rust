use std::mem;

// `log` is only borrowed, so we can't move it out, but we can leave an empty Vec behind.
fn drain_log(log: &mut Vec<String>) -> Vec<String> {
    mem::take(log)
}

fn main() {
    let mut log = vec![String::from("started"), String::from("stopped")];
    let text_before = log[0].as_ptr();

    let all = drain_log(&mut log);
    println!("all: {all:?}");
    println!("log now: {log:?}, length {}", log.len());
    println!("same text, not a copy: {}", all[0].as_ptr() == text_before);

    log.push(String::from("started again")); // the emptied Vec still works
    println!("log after push: {log:?}");
}
