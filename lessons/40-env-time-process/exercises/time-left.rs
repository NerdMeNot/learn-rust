use std::time::{Duration, Instant};

#[derive(Debug, PartialEq)]
enum Left {
    Remaining(Duration),
    UsedUp,
    ClockWentBack,
}

/// How much of `limit` is left at `now`, counting from `start`? `UsedUp` once the limit is used
/// up (exactly used up counts as used up). A `now` before `start` cannot happen with a good
/// clock, so report `ClockWentBack` rather than hiding it.
fn time_left(start: Instant, limit: Duration, now: Instant) -> Left {
    todo!(
        "Instant has checked_duration_since (an Option) and saturating_duration_since (zero instead); Duration has checked_sub. Which one reports a clock that went back?"
    )
}

/// The average of the samples, or `None` when there are none.
fn average(samples: &[Duration]) -> Option<Duration> {
    todo!("a Duration divides by a u32, and dividing by zero panics")
}

fn main() {
    // the instants are built by adding Durations, so no check depends on the real clock
    let start = Instant::now();
    let limit = Duration::from_secs(10);
    let at = |ms: u64| start + Duration::from_millis(ms);

    assert_eq!(time_left(start, limit, at(0)), Left::Remaining(limit));
    assert_eq!(
        time_left(start, limit, at(3000)),
        Left::Remaining(Duration::from_secs(7))
    );
    assert_eq!(
        time_left(start, limit, at(9500)),
        Left::Remaining(Duration::from_millis(500))
    );
    assert_eq!(time_left(start, limit, at(10_000)), Left::UsedUp);
    assert_eq!(time_left(start, limit, at(11_000)), Left::UsedUp);
    // a clock reading from before the start: duration_since and saturating_duration_since
    // would both say "nothing used", and the caller would never learn of the mistake
    assert_eq!(time_left(at(1000), limit, start), Left::ClockWentBack);

    assert_eq!(average(&[]), None);
    assert_eq!(
        average(&[Duration::from_secs(1), Duration::from_secs(2)]),
        Some(Duration::from_millis(1500))
    );
    assert_eq!(
        average(&[Duration::from_millis(250)]),
        Some(Duration::from_millis(250))
    );

    println!("all checks passed");
}
