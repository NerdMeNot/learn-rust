// Return the band of a temperature in whole degrees Celsius:
//   below 0          -> "freezing"
//   0 up to 15       -> "cold"
//   16 up to 25      -> "mild"
//   26 and above     -> "hot"
// Use an `if` / `else if` chain as the function's value (no `return`).
fn band(celsius: i32) -> &'static str {
    if celsius < 0 {
        "freezing"
    } else if celsius <= 15 {
        "cold"
    } else if celsius <= 25 {
        "mild"
    } else {
        "hot"
    }
}

fn main() {
    assert_eq!(band(-1), "freezing");
    assert_eq!(band(0), "cold");
    assert_eq!(band(15), "cold");
    assert_eq!(band(16), "mild");
    assert_eq!(band(25), "mild");
    assert_eq!(band(26), "hot");
    assert_eq!(band(i32::MIN), "freezing");
    assert_eq!(band(i32::MAX), "hot");
    println!("all checks passed");
}
