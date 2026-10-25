fn count(m: &HashMap<String, i32>) -> usize {
    m.len()
}

fn main() {
    println!("count: {}", count(&Default::default()));
}
