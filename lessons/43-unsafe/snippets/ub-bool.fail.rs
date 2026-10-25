// SAFETY: none. Only 0 and 1 are valid bools, so this breaks the rule for `transmute`.
const FLAG: bool = unsafe { std::mem::transmute::<u8, bool>(3) };

fn main() {
    println!("{FLAG}");
}
