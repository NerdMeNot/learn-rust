// An integration test: it uses the crate the way the binary does, through its public API.
use tally::{parse, total_cents};

#[test]
fn totals_the_sample_file() {
    let orders = parse(include_str!("../orders.csv")).unwrap();
    assert_eq!(total_cents(&orders), 3 * 40 + 250 + 2 * 575);
}
