fn add_item(item: &str, bucket: Option<Vec<String>>) -> Vec<String> {
    let mut bucket = bucket.unwrap_or_default();
    bucket.push(item.to_string());
    bucket
}

fn main() {
    println!("{:?}", add_item("a"));
}
