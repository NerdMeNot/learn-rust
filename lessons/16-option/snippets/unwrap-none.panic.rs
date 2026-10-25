fn find_age(id: u32) -> Option<u32> {
    if id == 1 { Some(31) } else { None }
}

fn main() {
    println!("user 1: {}", find_age(1).unwrap());
    println!("user 2: {}", find_age(2).unwrap());
}
