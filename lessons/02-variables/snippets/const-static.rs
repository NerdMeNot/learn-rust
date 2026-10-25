const SECONDS_PER_DAY: u32 = 24 * 60 * 60;
static APP_NAME: &str = "tracker";

fn main() {
    const LIMIT: u32 = 3; // a const may sit inside a function
    let days = 2;
    let seconds = days * SECONDS_PER_DAY;
    println!("{APP_NAME}: {seconds} seconds in {days} days (limit {LIMIT})");
}
