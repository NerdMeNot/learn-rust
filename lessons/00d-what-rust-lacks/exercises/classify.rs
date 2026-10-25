// A subject-less switch (or an if / elif chain):
//
//   switch {
//   case n < 0:  return "negative"
//   case n == 0: return "zero"
//   case n < 10: return "small"
//   default:     return "large"
//   }
//
// Write it as one match with guards: an arm can end in `if condition`.
fn classify(n: i32) -> &'static str {
    todo!("a match with guard arms: _ if n < 0 => ...")
}

fn main() {
    assert_eq!(classify(-5), "negative");
    assert_eq!(classify(0), "zero");
    assert_eq!(classify(9), "small");
    assert_eq!(classify(10), "large");
    assert_eq!(classify(i32::MAX), "large");
    println!("all checks passed");
}
