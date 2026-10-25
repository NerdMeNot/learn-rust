use std::fmt;

// Goal: print scores as "scores: 3, 5, 9 (total 17)", or "scores: none" when empty.
//
// Step 1. Uncomment this block, put any body in it, and run. Read the error
// (E0117): Display is not yours and Vec<u32> is not yours.
//
// impl fmt::Display for Vec<u32> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "scores")
//     }
// }
//
// Step 2. Comment it out again and write the impl for the wrapper below,
// a type that IS yours.
struct Scores(Vec<u32>);

impl fmt::Display for Scores {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            return write!(f, "scores: none");
        }
        let parts: Vec<String> = self.0.iter().map(|n| n.to_string()).collect();
        let total: u32 = self.0.iter().sum();
        write!(f, "scores: {} (total {})", parts.join(", "), total)
    }
}

fn main() {
    assert_eq!(
        Scores(vec![3, 5, 9]).to_string(),
        "scores: 3, 5, 9 (total 17)"
    );
    assert_eq!(Scores(vec![4]).to_string(), "scores: 4 (total 4)");
    // Nothing to join, nothing to total.
    assert_eq!(Scores(vec![]).to_string(), "scores: none");
    // The wrapper still holds a normal Vec.
    assert_eq!(Scores(vec![1, 2]).0.len(), 2);
    println!("all checks passed");
}
