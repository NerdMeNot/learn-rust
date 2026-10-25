async fn load_count() -> u32 {
    3
}

async fn report() -> u32 {
    let n: u32 = load_count();
    n + 1
}

fn main() {
    let _ = report();
}
