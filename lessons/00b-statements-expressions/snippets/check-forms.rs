// Backs the "you write -> Rust" tables: every Rust form below is run, and the values
// are the ones the other languages' check-forms programs print.
fn log(msg: &str) {
    println!("log: {msg}");
}

fn main() {
    let n = 7;

    // c ? a : b
    let size = if n > 5 { "big" } else { "small" };
    println!("ternary: {size}");

    // a value from a block, with a statement before it
    let area = {
        let w = 3;
        let h = 4;
        w * h
    };
    println!("block: {area}");

    // switch / when / match as a value
    let name = match 2 {
        1 => "one",
        2 => "two",
        _ => "many",
    };
    println!("switch: {name}");

    // a switch arm that is a block ending in a value (Java: `yield`)
    let area_name = match 2 {
        1 => "one".to_string(),
        2 => {
            let w = 3;
            let h = 4;
            format!("area {}", w * h)
        }
        _ => "many".to_string(),
    };
    println!("yield: {area_name}");

    // a, b = b, a
    let mut a = 1;
    let mut b = 2;
    (a, b) = (b, a);
    println!("swap: {a} {b}");

    // the comma operator: evaluate the first, keep the second
    let c = {
        log("first");
        2
    };
    println!("comma: {c}");

    // a closure body in braces needs no `return`
    let add_one = |x: i32| {
        let y = x + 1;
        y
    };
    println!("closure: {}", add_one(41));

    // a block-like expression in a `let` can be continued; as a statement it ends at `}`
    let t = if n > 0 { 10 } else { 20 } + 1;
    println!("let continues: {t}");
}
