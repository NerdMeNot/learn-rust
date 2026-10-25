fn tail(n: i32) -> i32 {
    n * 2
}

fn main() {
    // 1  a block is worth its last line
    let a = {
        let w = 3;
        w * 4
    };
    // 2  a trailing ; makes it worth ()
    let b = {
        let w = 3;
        let _ = w * 4;
    };
    // 3  if, match and loop are expressions
    let c = if a > 10 { "big" } else { "small" };
    let d = match a {
        12 => "twelve",
        _ => "other",
    };
    let mut k = 0;
    let e = loop {
        k += 1;
        if k == 3 {
            break k * 10;
        }
    };
    // 4  assignment is worth ()
    let mut m = 0;
    let f = m += 5;
    // 5  the function body is a block: no return needed
    println!("1  {a}");
    println!("2  {b:?}");
    println!("3  {c} {d} {e}");
    println!("4  {f:?} {m}");
    println!("5  {}", tail(4));
}
