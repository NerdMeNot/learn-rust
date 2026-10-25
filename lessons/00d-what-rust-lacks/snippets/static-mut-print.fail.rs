static mut VISITS: u32 = 0;

fn main() {
    unsafe {
        VISITS += 1;
        println!("{}", VISITS);
    }
}
