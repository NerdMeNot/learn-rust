use std::fmt;

// Releases sort by major, then minor, then patch.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Release {
    patch: u32,
    minor: u32,
    major: u32,
}

// "v1.2.3", and it must obey a width like {:>8} the way a number does.
impl fmt::Display for Release {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("build the text, then hand it to f.pad")
    }
}

fn release(major: u32, minor: u32, patch: u32) -> Release {
    Release {
        major,
        minor,
        patch,
    }
}

fn main() {
    let mut all = vec![
        release(1, 9, 1),
        release(2, 0, 0),
        release(1, 10, 0),
        release(1, 9, 0),
    ];
    all.sort();
    let sorted: Vec<String> = all.iter().map(|r| r.to_string()).collect();
    assert_eq!(sorted, ["v1.9.0", "v1.9.1", "v1.10.0", "v2.0.0"]);
    assert!(release(1, 10, 0) > release(1, 9, 99));

    let r = release(1, 2, 3);
    assert_eq!(format!("{r}"), "v1.2.3");
    assert_eq!(format!("[{r:>8}]"), "[  v1.2.3]");
    assert_eq!(format!("[{r:<8}]"), "[v1.2.3  ]");
    assert_eq!(format!("[{r:^8}]"), "[ v1.2.3 ]");
    println!("all checks passed");
}
