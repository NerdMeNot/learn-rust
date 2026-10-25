use std::mem;

// Backs the "what you want" table in "Moving out of a borrow".
fn main() {
    // look at the text inside an Option<String>, keep the Option as it was
    let opt = Some(String::from("abc"));
    assert_eq!(opt.as_deref(), Some("abc"));
    assert_eq!(opt.as_ref().map(|s| s.len()), Some(3));
    assert_eq!(opt, Some(String::from("abc")));

    // take the value, leave None
    let mut opt = Some(String::from("abc"));
    let taken = opt.take();
    assert_eq!(taken, Some(String::from("abc")));
    assert_eq!(opt, None);

    // take a whole String or Vec, leave an empty one
    let mut text = String::from("abc");
    let old = mem::take(&mut text);
    assert_eq!((old.as_str(), text.as_str()), ("abc", ""));
    let mut list = vec![1, 2];
    let old = mem::take(&mut list);
    assert_eq!((old, list), (vec![1, 2], vec![]));

    // take the old value, put a chosen one in
    let mut text = String::from("abc");
    let old = mem::replace(&mut text, String::from("xyz"));
    assert_eq!((old.as_str(), text.as_str()), ("abc", "xyz"));

    // a clone is a second value; the original is still there
    let text = String::from("abc");
    let copy = text.clone();
    assert_eq!(text, copy);
    assert_ne!(text.as_ptr(), copy.as_ptr());
}
