fn main() {
    // a step that is not +1 is a `while` with the step written by hand
    print!("doubling:");
    let mut i = 1;
    while i < 100 {
        print!(" {i}");
        i *= 2;
    }
    println!();

    // `continue` skips the rest of the pass, including a step written at the end
    let mut j = 0;
    let mut passes = 0;
    while j < 3 && passes < 6 {
        // (the `passes` guard only stops this demo)
        passes += 1;
        if j == 1 {
            continue;
        }
        j += 1;
    }
    println!("while: j = {j}, passes = {passes}");

    // the same goes for the test at the end of a `loop` that stands in for `do ... while`
    let mut k = 0;
    let mut passes = 0;
    loop {
        k += 1;
        passes += 1;
        if passes == 6 {
            break; // guard, only for this demo
        }
        if k >= 3 {
            continue; // in C, `continue` in a do-while goes on to the test
        }
        if !(k < 3) {
            break;
        }
    }
    println!("loop: k = {k}, passes = {passes}");
}
