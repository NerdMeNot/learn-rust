// A C switch that falls through, and nothing else:
//
//   int mask = 0;
//   switch (level) {
//   case 3: mask |= 4;
//   case 2: mask |= 2;
//   case 1: mask |= 1; break;
//   }
//
// Rust arms never fall through into the next one. Return the same mask.
fn mask(level: u32) -> u32 {
    todo!("one match arm per level, each giving its own total")
}

fn main() {
    assert_eq!(mask(3), 7);
    assert_eq!(mask(2), 3);
    assert_eq!(mask(1), 1);
    assert_eq!(mask(0), 0);
    assert_eq!(mask(4), 0);
    println!("all checks passed");
}
