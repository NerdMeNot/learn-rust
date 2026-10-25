struct Guard<'a>(&'a mut Vec<i32>);

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        println!("guard dropped");
    }
}

fn main() {
    let mut v = vec![1];
    let guard = Guard(&mut v);
    println!("last use of guard: {}", guard.0.len());
    v.push(2); // `guard` is not used again, but its drop runs at the end of `main`
}
