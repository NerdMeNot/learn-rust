use std::thread;

// Runs one failing thread to the end and reports what join gave back.
fn report(name: &str, handle: thread::JoinHandle<i32>) {
    match handle.join() {
        Ok(n) => println!("{name}: returned {n}"),
        Err(payload) => {
            let as_str = payload.downcast_ref::<&str>().is_some();
            let as_string = payload.downcast_ref::<String>().is_some();
            println!("{name}: panicked; payload is &str: {as_str}, String: {as_string}");
        }
    }
}

fn main() {
    // fixed text is a &str; so is a "formatted" message the compiler can fold into fixed text
    report("literal", thread::spawn(|| panic!("worker failed")));
    report("folded", thread::spawn(|| panic!("boom {}", 1)));
    // a message built from a value only known at run time is a String
    report(
        "built",
        thread::spawn(|| {
            let v: Vec<i32> = Vec::new();
            v[0]
        }),
    );
    println!("main carries on");
}
