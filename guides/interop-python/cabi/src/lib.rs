/// A plain C function: no Python anywhere.
#[unsafe(no_mangle)]
pub extern "C" fn wordstats_add(a: i64, b: i64) -> i64 {
    a + b
}
