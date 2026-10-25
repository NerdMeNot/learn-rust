#[derive(Debug, Clone, PartialEq, Default)]
struct Settings {
    name: String,
    volume: u8,
    muted: bool,
}

/// Same settings under a new name. `s` is used up.
fn rename(s: Settings, name: &str) -> Settings {
    Settings {
        name: name.to_string(),
        ..s
    }
}

/// A copy of `s` with the volume halved (rounding down) and `muted` set to false.
/// `s` itself is left as it was.
fn quieter(s: &Settings) -> Settings {
    Settings {
        volume: s.volume / 2,
        muted: false,
        ..s.clone()
    }
}

/// Settings with this name and every other field at its default (0 and false).
fn fresh(name: &str) -> Settings {
    Settings {
        name: name.to_string(),
        ..Default::default()
    }
}

fn main() {
    let lounge = Settings {
        name: String::from("lounge"),
        volume: 9,
        muted: true,
    };
    let q = quieter(&lounge);
    assert_eq!(
        q,
        Settings {
            name: String::from("lounge"),
            volume: 4,
            muted: false
        }
    );
    // The original is untouched.
    assert_eq!(lounge.volume, 9);
    assert!(lounge.muted);
    assert_eq!(lounge.name, "lounge");

    let blank = fresh("attic");
    assert_eq!(blank.name, "attic");
    assert_eq!(blank.volume, 0);
    assert!(!blank.muted);
    assert_eq!(fresh("").name, "");

    let kitchen = rename(lounge, "kitchen");
    assert_eq!(kitchen.name, "kitchen");
    assert_eq!(kitchen.volume, 9);
    assert!(kitchen.muted);

    // Edge cases: volume 1 halves to 0, and names can be empty.
    let tiny = Settings {
        name: String::new(),
        volume: 1,
        muted: false,
    };
    assert_eq!(quieter(&tiny).volume, 0);
    assert_eq!(quieter(&tiny).name, "");
    assert_eq!(rename(tiny, "").name, "");

    println!("all checks passed");
}
