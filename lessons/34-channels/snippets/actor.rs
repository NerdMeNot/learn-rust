use std::collections::HashMap;
use std::sync::mpsc::{self, Sender};
use std::thread;

// Messages are an enum: the usual way to send "several kinds of thing".
enum Cmd {
    Set(String, i32),
    // A request carries a Sender for the reply.
    Get(String, Sender<Option<i32>>),
    Divide(i32, Sender<Result<i32, String>>), // errors cross threads as values
}

fn main() {
    let (tx, rx) = mpsc::channel();

    // This thread owns the map. No other thread can reach it, so there is no lock.
    let owner = thread::spawn(move || {
        let mut map = HashMap::new();
        for cmd in rx {
            match cmd {
                Cmd::Set(k, v) => {
                    map.insert(k, v);
                }
                Cmd::Get(k, reply) => reply.send(map.get(&k).copied()).unwrap(),
                Cmd::Divide(d, reply) => {
                    let total: i32 = map.values().sum();
                    let r = if d == 0 {
                        Err("divide by zero".to_string())
                    } else {
                        Ok(total / d)
                    };
                    reply.send(r).unwrap();
                }
            }
        }
        map.len() // the loop ends when every Sender is dropped
    });

    tx.send(Cmd::Set("a".into(), 10)).unwrap();
    tx.send(Cmd::Set("b".into(), 20)).unwrap();

    let (reply_tx, reply_rx) = mpsc::channel();
    tx.send(Cmd::Get("a".into(), reply_tx.clone())).unwrap();
    println!("get a: {:?}", reply_rx.recv().unwrap());
    tx.send(Cmd::Get("z".into(), reply_tx)).unwrap();
    println!("get z: {:?}", reply_rx.recv().unwrap());

    let (rtx, rrx) = mpsc::channel();
    tx.send(Cmd::Divide(5, rtx.clone())).unwrap();
    println!("sum / 5: {:?}", rrx.recv().unwrap());
    tx.send(Cmd::Divide(0, rtx)).unwrap();
    println!("sum / 0: {:?}", rrx.recv().unwrap());

    drop(tx);
    println!("entries when the owner ended: {}", owner.join().unwrap());
}
