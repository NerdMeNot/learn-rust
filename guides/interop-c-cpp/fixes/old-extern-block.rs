extern "C" {
    fn abs(x: i32) -> i32;
}

pub fn absolute(x: i32) -> i32 {
    unsafe { abs(x) }
}
