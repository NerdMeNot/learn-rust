// Indentation is not part of the grammar: this is one function, laid out badly on purpose.
#[rustfmt::skip]
fn main() {
let a = 1;
        let b   =   a + 1;
    if b == 2 {
  assert_eq!(a + b, 3);
        }
}
