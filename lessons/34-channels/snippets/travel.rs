use std::mem::size_of;
use std::sync::mpsc;
use std::thread;

fn main() {
    // A Copy value: send copies it, so n is still usable.
    let (tx, rx) = mpsc::channel();
    let n = 7;
    tx.send(n).unwrap();
    println!("n after send: {n}, received: {}", rx.recv().unwrap());

    // A Vec: send moves it. The Vec is 3 words; its numbers stay where they were.
    let (tx, rx) = mpsc::channel::<Vec<i32>>();
    let v = vec![10, 20, 30];
    let before = v.as_ptr();
    println!("size_of::<Vec<i32>>(): {} bytes", size_of::<Vec<i32>>());

    tx.send(v).unwrap(); // v is moved: using it here would not compile
    let got = thread::spawn(move || rx.recv().unwrap()).join().unwrap();

    println!("received: {got:?}");
    println!(
        "same heap buffer after the trip: {}",
        got.as_ptr() == before
    );
}
