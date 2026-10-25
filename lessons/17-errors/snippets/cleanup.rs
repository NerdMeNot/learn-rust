use std::num::ParseIntError;

// Drop is code that runs when a value goes out of scope (the Traits lesson
// covers it). It plays the part of finally, a destructor, or a deferred call.
struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("cleanup: {}", self.0);
    }
}

fn work(text: &str) -> Result<u32, ParseIntError> {
    let _guard = Guard("work");
    let n: u32 = text.parse()?; // on an Err, the function returns from here
    println!("parsed {n}");
    Ok(n)
}

fn main() {
    println!("work(\"7\"): {:?}", work("7"));
    println!("work(\"x\"): {:?}", work("x"));
}
