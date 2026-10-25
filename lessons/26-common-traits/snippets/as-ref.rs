// &str: read-only, and a caller with a String must write &name.
fn shout_ref(s: &str) -> usize {
    s.len()
}

// AsRef<str>: read-only, and the caller may pass a &str or a String as they are.
fn shout_asref(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

// Into<String>: the function keeps the text, so it needs an owned String.
fn keep(s: impl Into<String>) -> String {
    s.into()
}

fn main() {
    let owned = String::from("rust");
    println!("&str: {} {}", shout_ref("rust"), shout_ref(&owned));
    println!("AsRef: {} {}", shout_asref("rust"), shout_asref(&owned));
    println!("AsRef moved the String: {}", shout_asref(owned));
    println!("Into: {} {}", keep("rust"), keep(String::from("go")));
}
