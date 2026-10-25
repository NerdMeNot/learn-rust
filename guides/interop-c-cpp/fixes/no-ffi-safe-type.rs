#![deny(improper_ctypes_definitions)]

#[unsafe(no_mangle)]
pub extern "C" fn first_word(text: &str) -> usize {
    text.split(' ').next().map_or(0, str::len)
}

#[unsafe(no_mangle)]
pub extern "C" fn total(v: Vec<i32>) -> i32 {
    v.iter().sum()
}
