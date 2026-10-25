static LIMIT: u32 = 10;

fn main() {
    let spent = 12;
    match spent {
        LIMIT => println!("at the limit"),
        _ => println!("elsewhere"),
    }
}
