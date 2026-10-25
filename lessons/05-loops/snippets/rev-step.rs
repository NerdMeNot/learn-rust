fn main() {
    // the order matters: step_by counts from the front of whatever it is given
    print!("rev then step_by(3):");
    for i in (1..10).rev().step_by(3) {
        print!(" {i}");
    }
    println!();

    print!("step_by(3) then rev:");
    for i in (1..10).step_by(3).rev() {
        print!(" {i}");
    }
    println!();
}
