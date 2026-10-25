/// The nickname if there is one that is not empty, otherwise the real name.
fn display_name(nickname: &Option<String>, real: &str) -> String {
    todo!("as_deref turns &Option<String> into Option<&str>; then filter, then unwrap_or")
}

fn main() {
    assert_eq!(display_name(&Some(String::from("bo")), "Robert"), "bo");
    assert_eq!(display_name(&None, "Robert"), "Robert");
    // An empty nickname counts as no nickname.
    assert_eq!(display_name(&Some(String::new()), "Robert"), "Robert");
    // The input is only borrowed, so it is still usable afterwards.
    let kept = Some(String::from("bo"));
    assert_eq!(display_name(&kept, ""), "bo");
    assert_eq!(kept, Some(String::from("bo")));

    println!("all checks passed");
}
