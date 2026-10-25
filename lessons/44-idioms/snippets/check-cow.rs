use std::borrow::Cow;

/// Borrows the input when there is nothing to change, and allocates only when there is.
fn no_tabs(text: &str) -> Cow<'_, str> {
    if text.contains('\t') {
        Cow::Owned(text.replace('\t', "    "))
    } else {
        Cow::Borrowed(text)
    }
}

fn main() {
    let clean = no_tabs("no tabs here");
    let fixed = no_tabs("a\tb");
    assert!(matches!(clean, Cow::Borrowed(_)));
    assert!(matches!(fixed, Cow::Owned(_)));
    assert_eq!(fixed, "a    b");
    println!("clean is borrowed, fixed is owned");
}
