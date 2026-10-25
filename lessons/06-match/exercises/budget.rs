// Describe how much of a limit has been spent.
//   nothing spent          -> "unused"
//   more than the limit    -> "over by N"
//   exactly the limit      -> "exactly at the limit"
//   1 to 9 (under limit)   -> "a little (N)"
//   anything else          -> "N of LIMIT used"
// A pattern cannot compare with the variable `limit`, so the limit checks are guards.
fn budget(spent: u32, limit: u32) -> String {
    todo!(
        "a match on spent: a literal arm, guards that use limit, an @ range, then a name for the rest"
    )
}

fn main() {
    assert_eq!(budget(0, 100), "unused");
    // The first arm wins, even when the limit is also 0.
    assert_eq!(budget(0, 0), "unused");
    assert_eq!(budget(150, 100), "over by 50");
    assert_eq!(budget(100, 100), "exactly at the limit");
    assert_eq!(budget(5, 100), "a little (5)");
    assert_eq!(budget(50, 100), "50 of 100 used");
    // Over the limit beats "a little".
    assert_eq!(budget(5, 3), "over by 2");
    assert_eq!(budget(3, 3), "exactly at the limit");
    println!("all checks passed");
}
