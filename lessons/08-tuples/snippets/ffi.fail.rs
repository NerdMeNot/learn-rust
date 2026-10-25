// Turned into an error here to show the message; by default it is a warning.
#![deny(improper_ctypes_definitions)]

pub extern "C" fn pair() -> (i32, i32) {
    (3, 4)
}

fn main() {
    println!("{:?}", pair());
}
