// "baby" for 0, "child (n)" for 1 to 12, "teen (n)" for 13 to 19, "adult (n)" for 20 and up.
fn age_group(age: u32) -> String {
    todo!("match age; \"baby\" needs String::from; `n @ 1..=12 =>` keeps the value as n")
}

fn main() {
    assert_eq!(age_group(0), "baby");
    assert_eq!(age_group(1), "child (1)");
    assert_eq!(age_group(12), "child (12)");
    assert_eq!(age_group(13), "teen (13)"); // the edge between two ranges
    assert_eq!(age_group(19), "teen (19)");
    assert_eq!(age_group(20), "adult (20)");
    assert_eq!(age_group(99), "adult (99)");
    println!("all checks passed");
}
