//! Runs the real `shop` program, the way a user would.

use std::io::Write;
use std::process::{Command, Output, Stdio};

fn shop(args: &[&str], stdin: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_shop"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(stdin.as_bytes()).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn a_script_runs_against_memory() {
    let out = shop(&[], "PUT TEA 450 Tea\nTOTAL TEA:3\n");
    assert_eq!(String::from_utf8_lossy(&out.stdout), "201 stored\n200 13.50\n");
    assert!(out.status.success());
}

#[test]
fn a_failed_request_sets_the_exit_code() {
    let out = shop(&["GET", "TEA"], "");
    assert_eq!(String::from_utf8_lossy(&out.stdout), "404 no product TEA\n");
    assert_eq!(out.status.code(), Some(1));
}

#[cfg(not(feature = "file"))]
#[test]
fn db_is_refused_without_the_feature() {
    let out = shop(&["--db", "x.jsonl", "LIST"], "");
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("`file` feature"));
}

#[cfg(feature = "file")]
#[test]
fn products_survive_between_runs() {
    let db = std::env::temp_dir().join(format!("shop-cli-{}.jsonl", std::process::id()));
    let db_arg = db.to_str().unwrap();
    let _ = std::fs::remove_file(&db);
    shop(&["--db", db_arg, "PUT", "MUG", "1200", "Enamel", "mug"], "");
    let out = shop(&["--db", db_arg, "GET", "MUG"], "");
    assert_eq!(String::from_utf8_lossy(&out.stdout), "200 MUG 12.00 Enamel mug\n");
    std::fs::remove_file(&db).unwrap();
}
