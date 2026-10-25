struct Point {
    x: i32,
    y: i32,
}

enum Value {
    Text(String),
    Pt(Point),
    Other,
}

fn describe(v: &Value) -> String {
    match v {
        Value::Text(s) if s.len() > 3 => format!("long text {}", s.len()),
        Value::Pt(Point { x, .. }) => format!("point at {x}"),
        _ => "other".to_string(),
    }
}

fn main() {
    let Point { y, x } = Point { x: 1, y: 2 };
    println!("x={x} y={y}");
    println!("{}", describe(&Value::Text("hello".to_string())));
    println!("{}", describe(&Value::Pt(Point { x: 5, y: 6 })));
    println!("{}", describe(&Value::Other));
}
