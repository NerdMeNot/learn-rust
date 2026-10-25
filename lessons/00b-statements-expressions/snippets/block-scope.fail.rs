fn main() {
    let area = {
        let w = 3;
        let h = 4;
        w * h
    };
    println!("area: {area}");
    println!("w after the block: {w}");
}
