use std::process::Command;

// Cargo builds the `wordstat` binary for integration tests and tells us where it is.
fn wordstat(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_wordstat"))
        .args(args)
        .output()
        .expect("could not start wordstat")
}

#[test]
fn prints_the_most_common_word_first() {
    let out = wordstat(&["a", "b", "b"]);
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout), "  2 b\n  1 a\n");
}

#[test]
fn no_words_is_a_failure() {
    let out = wordstat(&[]);
    assert!(!out.status.success());
}
