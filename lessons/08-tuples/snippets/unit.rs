use std::mem::size_of;

fn log(msg: &str) {
    println!("log: {msg}");
}

fn check(ok: bool) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err("not ok".to_string())
    }
}

fn main() {
    // A function with no `->` returns the empty tuple.
    let nothing = log("hello");
    println!("nothing: {nothing:?}");
    println!("size_of::<()>(): {}", size_of::<()>());

    // `Result<(), E>`: "it worked, there is nothing to return".
    println!("check(true): {:?}", check(true));
    println!("check(false): {:?}", check(false));

    // A one-slot tuple needs the comma.
    let one = (5,);
    println!("one: {one:?}, one.0: {}", one.0);
}
