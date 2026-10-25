use std::hint::black_box;

fn main() {
    println!("debug assertions on: {}", cfg!(debug_assertions));
    // black_box hides the value from the optimiser, so this is checked at run time.
    let x: u8 = black_box(255);
    println!("wrapping_add: {}", x.wrapping_add(1));
    println!("checked_add:  {:?}", x.checked_add(1));
    println!("x + 1:        {}", x + 1);
}
