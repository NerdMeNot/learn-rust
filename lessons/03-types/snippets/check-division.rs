fn main() {
    assert_eq!(i32::MIN.checked_div(-1), None);
    assert_eq!(5_i32.checked_div(0), None);
    assert_eq!(5_i32.checked_rem(0), None);
    assert_eq!((-7_i32).rem_euclid(2), 1);
    // A method call binds tighter than unary minus: this is -(7.rem_euclid(2)).
    assert_eq!(-7_i32.rem_euclid(2), -1);
    assert_eq!(7_i32.rem_euclid(-2), 1);
    assert_eq!((-7.5_f64).rem_euclid(2.0), 0.5);
    assert_eq!(-7.5_f64 % 2.0, -1.5);
    println!("all division checks passed");
}
