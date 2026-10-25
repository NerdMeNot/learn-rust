const LIMIT: i32 = 10;

fn main() {
    let limit = 10;
    for n in [3, 10, 42] {
        match n {
            LIMIT => println!("{n}: at the limit (const)"),
            n if n > limit => println!("{n}: over the limit (guard)"),
            _ => println!("{n}: under the limit"),
        }
    }
}
