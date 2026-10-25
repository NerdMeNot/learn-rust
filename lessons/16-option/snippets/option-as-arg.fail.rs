fn find_age(id: u32) -> Option<u32> {
    if id == 1 { Some(31) } else { None }
}

fn main() {
    let age: u32 = find_age(1);
    println!("age: {age}");
}
