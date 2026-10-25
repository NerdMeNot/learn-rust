//! Tests that run the real `todo` binary, the way a user does.
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

/// A private folder for one test's list file, removed when the test ends.
struct Sandbox {
    dir: PathBuf,
}

impl Sandbox {
    fn new(name: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("todo-cli-{}-{name}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        Sandbox { dir }
    }

    fn file(&self) -> PathBuf {
        self.dir.join("todo.json")
    }

    /// Run `todo --file <this sandbox's file> <args...>`.
    fn todo(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_todo"))
            .arg("--file")
            .arg(self.file())
            .args(args)
            .output()
            .expect("the binary starts")
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

#[test]
fn a_new_list_is_empty() {
    let sb = Sandbox::new("empty");
    let out = sb.todo(&["list"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out), "no tasks\n");
}

#[test]
fn added_tasks_are_listed_and_kept_between_runs() {
    let sb = Sandbox::new("add");
    let out = sb.todo(&["add", "write the page", "-p", "1"]);
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(stdout(&out), "added [ ]  1  p1  write the page\n");
    assert_eq!(stdout(&sb.todo(&["list"])), "[ ]  1  p1  write the page\n");
}

#[test]
fn done_tasks_can_be_hidden() {
    let sb = Sandbox::new("done");
    sb.todo(&["add", "one"]);
    sb.todo(&["add", "two"]);
    sb.todo(&["done", "1"]);
    assert_eq!(stdout(&sb.todo(&["list", "--open"])), "[ ]  2  p2  two\n");
}

#[test]
fn an_unknown_id_fails_with_a_message() {
    let sb = Sandbox::new("unknown-id");
    let out = sb.todo(&["done", "9"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        stderr(&out).contains("no task with id 9"),
        "{}",
        stderr(&out)
    );
}

#[test]
fn a_bad_priority_is_refused_and_nothing_is_saved() {
    let sb = Sandbox::new("bad-priority");
    let out = sb.todo(&["add", "x", "-p", "5"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("priority must be 1, 2 or 3, not 5"));
    assert!(!sb.file().exists());
}

#[test]
fn a_corrupt_file_is_reported_with_its_path() {
    let sb = Sandbox::new("corrupt");
    fs::write(sb.file(), "{oops").unwrap();
    let out = sb.todo(&["list"]);
    assert_eq!(out.status.code(), Some(1));
    let err = stderr(&out);
    assert!(err.contains("todo.json is not a valid to-do file"), "{err}");
    assert!(err.contains("Caused by"), "{err}");
}

#[test]
fn a_usage_error_exits_with_2() {
    let sb = Sandbox::new("usage");
    let out = sb.todo(&["add"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("Usage:"));
}
