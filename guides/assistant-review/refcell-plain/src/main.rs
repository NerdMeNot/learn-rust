/// Adds a follow-up message after every urgent one.
fn flag_urgent(messages: &mut Vec<String>) {
    for m in messages.iter() {
        if m.starts_with("URGENT") {
            messages.push(format!("follow up: {m}"));
        }
    }
}

fn main() {
    let mut messages = vec!["hello".to_string(), "URGENT: disk full".to_string()];
    flag_urgent(&mut messages);
    println!("{messages:?}");
}
