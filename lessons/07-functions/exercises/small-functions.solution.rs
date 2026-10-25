/// `n` limited to the range `lo..=hi`.
fn clamp(n: i32, lo: i32, hi: i32) -> i32 {
    if n < lo {
        return lo;
    }
    if n > hi {
        return hi;
    }
    n
}

/// Gregorian leap year: divisible by 4, except centuries, except every 400th year.
fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// The greatest common divisor, by Euclid's rule: gcd(a, 0) is a, otherwise gcd(b, a % b).
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn main() {
    assert_eq!(clamp(5, 0, 10), 5);
    assert_eq!(clamp(-3, 0, 10), 0);
    assert_eq!(clamp(99, 0, 10), 10);
    assert_eq!(clamp(0, 0, 10), 0);

    assert!(is_leap_year(2024));
    assert!(!is_leap_year(2023));
    // Centuries are not leap years, unless divisible by 400.
    assert!(!is_leap_year(1900));
    assert!(is_leap_year(2000));

    assert_eq!(gcd(12, 18), 6);
    assert_eq!(gcd(17, 5), 1);
    // The base case, whichever side is zero.
    assert_eq!(gcd(7, 0), 7);
    assert_eq!(gcd(0, 5), 5);

    println!("all checks passed");
}
