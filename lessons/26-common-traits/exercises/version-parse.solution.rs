use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
}

#[derive(Debug, PartialEq)]
struct ParseVersionError;

// "1.10" is major 1, minor 10. Anything else is an error:
// no dot, a third part, an empty part, or a part that is not plain digits (not even a sign).
impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Version, ParseVersionError> {
        let (major, minor) = s.split_once('.').ok_or(ParseVersionError)?;
        Ok(Version {
            major: digits(major)?,
            minor: digits(minor)?,
        })
    }
}

// u32::from_str accepts a leading `+`, so check for digits first.
fn digits(s: &str) -> Result<u32, ParseVersionError> {
    if !s.bytes().all(|b| b.is_ascii_digit()) {
        return Err(ParseVersionError);
    }
    s.parse().map_err(|_| ParseVersionError)
}

fn main() {
    let v: Version = "1.10".parse().unwrap();
    assert_eq!(
        v,
        Version {
            major: 1,
            minor: 10
        }
    ); // not 1.1
    assert_eq!(Version::from_str("0.0"), Ok(Version { major: 0, minor: 0 }));

    let old: Version = "1.9".parse().unwrap();
    assert!(old < v); // 9 < 10 as numbers

    for bad in [
        "", "1", "1.", ".2", "1.2.3", "a.b", "1.-2", "1.+2", "+1.2", "1,2",
    ] {
        assert_eq!(bad.parse::<Version>(), Err(ParseVersionError), "{bad:?}");
    }
    println!("all checks passed");
}
