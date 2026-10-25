#[derive(Debug, PartialEq, PartialOrd)]
struct Reading {
    sensor: u32,
    value: f64,
}

fn main() {
    let mut all = vec![
        Reading {
            sensor: 1,
            value: 2.5,
        },
        Reading {
            sensor: 2,
            value: 0.5,
        },
    ];
    all.sort();
    println!("{all:?}");
}
