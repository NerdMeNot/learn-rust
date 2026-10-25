#[derive(Debug, Clone, PartialEq, Default)]
struct Player {
    name: String,
    score: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Version {
    major: u32,
    minor: u32,
}

#[derive(PartialEq)]
struct Reading {
    celsius: f64,
}

fn main() {
    let a = Player {
        name: String::from("ana"),
        score: 10,
    };
    let b = a.clone();
    println!("a == b: {}", a == b);
    let c = Player {
        score: 11,
        ..a.clone()
    };
    println!("a == c: {}", a == c);
    println!("a != c: {}", a != c);
    println!("default: {:?}", Player::default());

    let v1 = Version { major: 1, minor: 9 };
    let v2 = Version { major: 2, minor: 0 };
    let v3 = Version {
        major: 1,
        minor: 10,
    };
    println!("1.9 < 2.0: {}", v1 < v2);
    println!("1.10 > 1.9: {}", v3 > v1);
    let copy = v1; // Copy: `v1` is not moved
    println!("v1 == copy: {}", v1 == copy);

    let r = Reading { celsius: f64::NAN };
    println!("NaN reading == itself: {}", r == r);
}
