use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now();
    println!("now is after the epoch: {}", now > UNIX_EPOCH);

    // a time built by hand: one day after 1970-01-01 00:00:00 UTC
    let next_day = UNIX_EPOCH + Duration::from_secs(86_400);
    let since = next_day
        .duration_since(UNIX_EPOCH)
        .expect("later than the epoch");
    println!("seconds since the epoch: {}", since.as_secs());

    // the wall clock can go backwards, so asking "how long since?" can fail
    match UNIX_EPOCH.duration_since(next_day) {
        Ok(d) => println!("positive: {d:?}"),
        Err(e) => println!("Err: {e}; it is off by {:?}", e.duration()),
    }
}
