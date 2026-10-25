use std::error::Error;
use std::mem::{size_of, size_of_val};
use std::num::ParseIntError;

fn main() {
    println!("size_of u32: {}", size_of::<u32>());
    println!("size_of ParseIntError: {}", size_of::<ParseIntError>());
    println!(
        "size_of Result<u32, ParseIntError>: {}",
        size_of::<Result<u32, ParseIntError>>()
    );
    println!("size_of &dyn Error: {}", size_of::<&dyn Error>());
    println!("size_of Box<dyn Error>: {}", size_of::<Box<dyn Error>>());
    println!(
        "size_of Result<(), Box<dyn Error>>: {}",
        size_of::<Result<(), Box<dyn Error>>>()
    );

    // The box holds the error value somewhere else. This is how big that value is.
    let boxed: Box<dyn Error> = Box::new("x".parse::<u32>().unwrap_err());
    println!(
        "size of the ParseIntError inside the box: {}",
        size_of_val(&*boxed)
    );
}
