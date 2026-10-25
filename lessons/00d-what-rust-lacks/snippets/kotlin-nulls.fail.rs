fn safe_call(name: Option<String>) -> usize {
    let n = name?.len();
    n
}

fn elvis(name: Option<String>) -> usize {
    name?.len() ?: 0
}

fn not_null() {
    let name: Option<String> = Some("a".to_string());
    let n = name!!.len();
}

fn main() {}
