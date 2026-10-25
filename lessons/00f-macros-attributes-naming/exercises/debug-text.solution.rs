#[derive(Debug)]
#[allow(dead_code)] // the fields are only printed, never read
struct Point {
    x: i32,
    y: i32,
}

/// The `Debug` text of an optional point: on one line, or spread over
/// several lines when `pretty` is true.
fn show(p: &Option<Point>, pretty: bool) -> String {
    if pretty {
        format!("{:#?}", p)
    } else {
        format!("{:?}", p)
    }
}

fn main() {
    let some = Some(Point { x: 1, y: -2 });
    assert_eq!(show(&some, false), "Some(Point { x: 1, y: -2 })");
    assert_eq!(
        show(&some, true),
        "Some(\n    Point {\n        x: 1,\n        y: -2,\n    },\n)"
    );
    // The case to watch: there may be no point at all.
    assert_eq!(show(&None, false), "None");
    assert_eq!(show(&None, true), "None");
    println!("all checks passed");
}
