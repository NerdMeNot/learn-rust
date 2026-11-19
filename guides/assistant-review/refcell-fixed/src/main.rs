/// Adds a follow-up message after every urgent one.
fn flag_urgent(messages: &mut Vec<String>) {
    let follow_ups: Vec<String> = messages
        .iter()
        .filter(|m| m.starts_with("URGENT"))
        .map(|m| format!("follow up: {m}"))
        .collect();
    messages.extend(follow_ups);
}

fn main() {
    let mut messages = vec!["hello".to_string(), "URGENT: disk full".to_string()];
    flag_urgent(&mut messages);
    println!("{messages:?}");
}
