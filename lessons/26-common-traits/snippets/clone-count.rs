struct Noisy(u32);

impl Clone for Noisy {
    fn clone(&self) -> Noisy {
        println!("cloning {}", self.0);
        Noisy(self.0)
    }
}

#[derive(Clone, Copy)]
struct Plain(u32);

fn main() {
    let a = Noisy(1);
    let b = a.clone();
    println!("b: {}", b.0);
    let c = b;
    println!("moved, no clone: {}", c.0);

    let list = vec![Noisy(10), Noisy(20)];
    println!("cloning the list:");
    let copy = list.clone();
    println!("copy has {} items", copy.len());

    let p = Plain(7);
    let q = p;
    println!("p and q both usable: {} {}", p.0, q.0);
}
