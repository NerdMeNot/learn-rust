fn clamp(x: i32) -> i32 {
    if x < 0 {
        return 0; // leaving early needs `return`
    }
    if x > 100 { 100 } else { x } // the last expression is the result
}

fn main() {
    let n = 7;
    let label = if n % 2 == 0 { "even" } else { "odd" };
    let area = {
        let w = 3;
        let h = 4;
        w * h // no semicolon: the block's value
    };
    println!("{n} is {label}");
    println!("area: {area}");
    println!("clamp: {} {} {}", clamp(-5), clamp(50), clamp(250));
}
