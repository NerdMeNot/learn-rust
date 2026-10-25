struct Excerpt<'a> {
    part: &'a str,
}

struct OwnedExcerpt {
    part: String,
}

fn main() {
    // Fix 1: declare the owner first, so it outlives the struct that borrows from it.
    let text = String::from("Call me Ishmael. Some years ago");
    let excerpt = Excerpt { part: &text[..15] };
    println!("borrowed: {}", excerpt.part);

    // Fix 3: keep a copy. The struct no longer depends on `temp`.
    let owned;
    {
        let temp = String::from("Call me Ishmael. Some years ago");
        owned = OwnedExcerpt {
            part: temp[..15].to_string(),
        };
    }
    println!("owned: {}", owned.part);
}
