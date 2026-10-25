fn shout(name: &str) -> (String, usize) {
    let upper = name.to_uppercase();
    let addr_inside = upper.as_ptr() as usize;
    (upper, addr_inside) // `upper` moves out to the caller
}

fn main() {
    let (s, addr_inside) = shout("rust");
    println!("s: {s}");
    println!(
        "same buffer inside and outside: {}",
        s.as_ptr() as usize == addr_inside
    );
    println!("len {}, capacity {}", s.len(), s.capacity());
}
