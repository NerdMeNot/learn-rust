struct Guard<'a>(&'a mut Vec<i32>);

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        println!("guard dropped");
    }
}

fn main() {
    let mut list = vec![1];
    let guard = Guard(&mut list);
    guard.0.push(5);
    list.push(2);
}
