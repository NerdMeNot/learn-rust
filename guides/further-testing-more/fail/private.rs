// tests/private.rs: an integration test is an outside user of the crate.
#[test]
fn unit_seconds_from_outside() {
    assert_eq!(timekeep::unit_seconds('m'), Some(60));
}
