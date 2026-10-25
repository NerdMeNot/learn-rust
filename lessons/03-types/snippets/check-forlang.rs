// Backs the "what you write today" tables: each Rust cell is compiled and run here.
use std::hint::black_box;
use std::panic::catch_unwind;

#[allow(unused_comparisons)]
fn main() {
    // Go: an unsigned counter is never below zero, so `i >= 0` cannot end a countdown.
    // The loop builds (with a warning), then panics when `i` goes below 0.
    let i: usize = black_box(0);
    assert!(i >= 0);
    let countdown = catch_unwind(|| {
        let mut i = black_box(2_usize);
        while i >= 0 {
            i -= 1;
        }
    });
    assert!(countdown.is_err());
    // Go: len is an int, and Rust's is a usize; Atoi maps to parse.
    let _: usize = "hello".len();
    assert_eq!("42".parse::<i64>(), Ok(42));
    assert_eq!(size_of::<usize>(), size_of::<*const u8>());
    assert_eq!(size_of::<isize>(), size_of::<usize>());
    // Go: wrap on purpose, and the same 44 as uint8(n).
    assert_eq!(i32::MAX.wrapping_add(1), i32::MIN);
    assert_eq!(black_box(300_i32) as u8, 44);
    // Go and TypeScript: truncating division and remainder.
    assert_eq!(-7 / 2, -3);
    assert_eq!(-7 % 2, -1);
    // Python, Java: floor division and floorMod versus div_euclid and rem_euclid.
    assert_eq!((-7_i32).div_euclid(2), -4);
    assert_eq!((-7_i32).rem_euclid(2), 1);
    assert_eq!(7_i32.rem_euclid(-2), 1);
    // Python and TypeScript: big values.
    assert_eq!(2_u128.pow(100), 1_267_650_600_228_229_401_496_703_205_376);
    assert!((1_i64 << 53) + 1 != 1_i64 << 53);
    assert_eq!(7 / 2, 3);
    assert_eq!(7.0 / 2.0, 3.5);
    assert_eq!(-3.9_f64 as i32, -3);
    // Java and Kotlin: floats saturate in both languages, and in Rust.
    assert_eq!(1e10_f64 as i32, 2_147_483_647);
    assert_eq!(f64::NAN as i32, 0);
    // The family names mentioned on the page exist.
    assert_eq!(5_u8.checked_sub(6), None);
    assert_eq!(200_u8.saturating_mul(2), 255);
    assert_eq!(10_i32.checked_pow(10), None);
    assert_eq!(i8::MIN.wrapping_neg(), i8::MIN);
    // usize::from exists for the small types (the page says u8, u16, bool).
    assert_eq!(
        usize::from(7_u8) + usize::from(7_u16) + usize::from(true),
        15
    );
    assert_eq!(usize::try_from(7_u32), Ok(7));
    // Division that overflows or divides by zero panics (the page shows the zero case).
    let min = black_box(i32::MIN);
    let neg_one = black_box(-1);
    let zero = black_box(0);
    assert!(catch_unwind(|| min / neg_one).is_err());
    assert!(catch_unwind(|| neg_one / zero).is_err());
    assert_eq!(min.checked_div(neg_one), None);
    println!("all language rows checked");
}
