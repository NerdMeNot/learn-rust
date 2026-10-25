// "baby" for 0, "child (n)" for 1 to 12, "teen (n)" for 13 to 19, "adult (n)" for 20 and up.
fn age_group(age: u32) -> String {
    match age {
        0 => String::from("baby"),
        n @ 1..=12 => format!("child ({n})"),
        n @ 13..=19 => format!("teen ({n})"),
        n => format!("adult ({n})"),
    }
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
