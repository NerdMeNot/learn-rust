fn main() {
    let mut passes = 0;
    // `mut i` is allowed, but it is a copy for this pass only
    for mut i in 0..5 {
        if i == 1 {
            i += 2;
        }
        passes += 1;
        println!("i: {i}");
    }
    println!("passes: {passes}");
}
