trait Speaker {
    fn speak(&self) -> String;
}

// A trait is not a type: its bare name is refused in all three places.
fn param(s: &Speaker) -> usize {
    s.speak().len()
}

fn list(v: Vec<Speaker>) -> usize {
    v.len()
}

fn result() -> Speaker {
    todo!()
}

fn main() {}
