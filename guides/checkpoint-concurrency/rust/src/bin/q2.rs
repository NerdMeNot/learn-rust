use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    tx.send(1).unwrap();
    drop(tx);
    println!("{:?}", rx.try_recv());
    println!("{:?}", rx.try_recv());
    drop(tx2);
    println!("{:?}", rx.try_recv());
}
