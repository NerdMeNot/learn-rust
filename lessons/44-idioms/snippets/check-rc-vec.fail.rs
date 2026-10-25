fn note(log: &mut Vec<String>, text: String) {
    log.push(text);
}

fn add_follow_ups(log: &mut Vec<String>) {
    for line in log.iter() {
        note(log, format!("follow up on {line}"));
    }
}

fn main() {
    let mut log = vec!["call Ada".to_string()];
    add_follow_ups(&mut log);
    println!("{log:?}");
}
