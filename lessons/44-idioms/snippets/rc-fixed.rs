fn add_follow_ups(log: &mut Vec<String>) {
    // Build the new notes first, then add them: the list is never read and changed at once.
    let follow_ups: Vec<String> = log
        .iter()
        .map(|line| format!("follow up on {line}"))
        .collect();
    log.extend(follow_ups);
}

fn main() {
    let mut log = vec!["call Ada".to_string()];
    add_follow_ups(&mut log);
    println!("{log:?}");
}
