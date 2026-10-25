fn helper() -> u32 {
    1
}

const fn f() -> u32 {
    helper()
}

fn main() {
    println!("{}", f());
}
