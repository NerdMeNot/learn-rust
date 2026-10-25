async fn read_count() -> u32 {
    3
}

fn total() -> u32 {
    read_count().await + 1
}

fn main() {
    let _ = total();
}
