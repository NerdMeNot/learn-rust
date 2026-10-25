mod common;

use common::{TempDir, dir_with_two_notes};
use std::panic::{AssertUnwindSafe, catch_unwind};
use timekeep::store::{load_note, note_names, save_note};

#[test]
fn saves_and_loads_a_note() {
    let dir = TempDir::new("roundtrip");
    let path = save_note(dir.path(), "todo", "write tests").unwrap();
    assert_eq!(load_note(&path).unwrap(), "write tests");
}

#[test]
fn lists_notes_in_sorted_order() {
    let dir = dir_with_two_notes();
    assert_eq!(note_names(dir.path()).unwrap(), ["review", "standup"]);
}

#[test]
fn missing_note_is_not_found() {
    let dir = TempDir::new("missing");
    let err = load_note(&dir.path().join("nope.txt")).unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
}

#[test]
fn temp_dir_is_removed_even_when_the_test_body_panics() {
    let mut kept = None;
    let result = catch_unwind(AssertUnwindSafe(|| {
        let dir = TempDir::new("panics");
        kept = Some(dir.path().to_path_buf());
        assert!(dir.path().exists());
        panic!("a failing assert would unwind the same way");
    }));
    assert!(result.is_err());
    assert!(!kept.unwrap().exists());
}
