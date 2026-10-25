// == between different types does not compile (Java's equals(Object) says false). Not shown on the page.
#[derive(PartialEq)]
struct Version(u32, u32);

fn main() {
    let v = Version(1, 2);
    println!("{}", v == "x");
}
