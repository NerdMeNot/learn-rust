use chrono::{Datelike, NaiveDate};
use rayon::prelude::*;
use regex::Regex;
use uuid::Uuid;

fn main() {
    // regex: pull the parts out of a date written in text
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let caps = re.captures("shipped on 2026-10-25, again").unwrap();
    println!("regex:  year {}, month {}, day {}", &caps[1], &caps[2], &caps[3]);

    // chrono and jiff: the same date in two date-time crates
    let d = NaiveDate::from_ymd_opt(2026, 10, 25).unwrap();
    println!("chrono: {}", d.weekday());
    let j = jiff::civil::date(2026, 10, 25);
    println!("jiff:   {:?}", j.weekday());

    // uuid: parse one and read its version number
    let id = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap();
    println!("uuid:   version {}", id.get_version_num());

    // rayon: the same sum a plain iterator gives, spread over threads
    let squares: u64 = (1..=100u64).into_par_iter().map(|n| n * n).sum();
    let plain: u64 = (1..=100u64).map(|n| n * n).sum();
    assert_eq!(squares, plain);
    println!("rayon:  {squares}");
}
