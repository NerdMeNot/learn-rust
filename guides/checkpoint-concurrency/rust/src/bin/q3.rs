use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;

fn main() {
    let n = AtomicUsize::new(5);
    println!("{}", n.fetch_add(1, SeqCst));
    println!("{}", n.load(SeqCst));
    println!("{:?}", n.compare_exchange(5, 10, SeqCst, SeqCst));
    println!("{:?}", n.compare_exchange(6, 10, SeqCst, SeqCst));
    println!("{}", n.load(SeqCst));
}
