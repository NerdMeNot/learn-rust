fn throws(n: i32) -> i32 {
    if n < 0 {
        throw "negative";
    }
    n
}

fn catches() {
    try {
        println!("work");
    } catch (e) {
        println!("failed");
    }
}

fn main() {}
