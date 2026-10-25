//! A session that expires, with the clock passed in so a test can control time.

use std::time::{SystemTime, UNIX_EPOCH};

/// Where "now" comes from.
pub trait Clock {
    /// Seconds since some fixed point.
    fn now(&self) -> u64;
}

/// The real clock: seconds since the Unix epoch.
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

/// A session that ends `ttl` seconds after it starts.
///
/// This example reads the real clock, so it is only compiled, not run:
///
/// ```no_run
/// use timekeep::session::{Session, SystemClock};
///
/// let session = Session::new(SystemClock, 3600);
/// assert!(!session.expired());
/// ```
///
/// A number is not a clock, and the compiler says so. `compile_fail` makes that the test:
///
/// ```compile_fail
/// let _ = timekeep::session::Session::new(5, 60);
/// ```
pub struct Session<C: Clock> {
    clock: C,
    started: u64,
    ttl: u64,
}

impl<C: Clock> Session<C> {
    pub fn new(clock: C, ttl: u64) -> Self {
        let started = clock.now();
        Session {
            clock,
            started,
            ttl,
        }
    }

    pub fn expired(&self) -> bool {
        self.clock.now() >= self.started + self.ttl
    }
}

/// How long a session lasts, in seconds: `SESSION_TTL` if the lookup finds it and it parses,
/// otherwise 300. The lookup is a parameter, so a test does not touch the real environment.
pub fn ttl_from(get: impl Fn(&str) -> Option<String>) -> u64 {
    get("SESSION_TTL")
        .and_then(|text| text.parse().ok())
        .unwrap_or(300)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::collections::HashMap;

    // The test's clock is a `Cell` the test can set. Implementing the trait for `&Cell<u64>`
    // lets the session borrow it while the test keeps changing the time.
    impl Clock for &Cell<u64> {
        fn now(&self) -> u64 {
            self.get()
        }
    }

    #[test]
    fn expires_after_the_ttl() {
        let now = Cell::new(1_000);
        let session = Session::new(&now, 60);
        assert!(!session.expired());
        now.set(1_059);
        assert!(!session.expired());
        now.set(1_060);
        assert!(session.expired());
    }

    #[test]
    fn ttl_comes_from_the_lookup() {
        let env: HashMap<&str, &str> = HashMap::from([("SESSION_TTL", "45")]);
        assert_eq!(ttl_from(|k| env.get(k).map(|v| v.to_string())), 45);
        assert_eq!(ttl_from(|_| None), 300);
        assert_eq!(ttl_from(|_| Some("soon".to_string())), 300);
    }
}
