// tests/env.rs: changing the environment from a test.
#[test]
fn reads_the_ttl_from_the_environment() {
    std::env::set_var("SESSION_TTL", "45");
    assert_eq!(std::env::var("SESSION_TTL").unwrap(), "45");
}
