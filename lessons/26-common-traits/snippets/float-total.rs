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
            value: f64::NAN,
        },
        Reading {
            sensor: 3,
            value: 0.5,
        },
        Reading {
            sensor: 4,
            value: -1.0,
        },
    ];
    let nan = f64::NAN;
    println!("nan < 1.0: {}", nan < 1.0);
    println!("nan > 1.0: {}", nan > 1.0);
    println!("nan == nan: {}", nan == nan);
    println!("partial_cmp: {:?}", nan.partial_cmp(&1.0));
    println!("total_cmp: {:?}", nan.total_cmp(&1.0));

    all.sort_by(|a, b| a.value.total_cmp(&b.value));
    for r in &all {
        println!("sensor {} value {}", r.sensor, r.value);
    }
}
