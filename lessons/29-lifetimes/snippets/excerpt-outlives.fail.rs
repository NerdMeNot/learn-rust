struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let excerpt;
    {
        let text = String::from("Call me Ishmael. Some years ago");
        excerpt = Excerpt { part: &text[..15] };
    }
    println!("excerpt: {}", excerpt.part);
}
