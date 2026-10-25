use std::future::ready;
struct Guard;
impl Drop for Guard {
    fn drop(&mut self) {
        ready(()).await;
    }
}
fn main() {
    let _g = Guard;
}
