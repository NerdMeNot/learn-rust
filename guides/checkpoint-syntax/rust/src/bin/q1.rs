fn main() {
    let x = 5;
    let x = {
        let x = x * 2;
        x + 1
    };
    let y = if x > 10 { "big" } else { "small" };
    let z = {
        let _unused = x + 1;
    };
    println!("{x} {y} {z:?}");
}
