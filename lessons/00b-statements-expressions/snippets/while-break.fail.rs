fn main() {
    let mut i = 0;
    let found = while i < 10 {
        i += 1;
        if i == 4 {
            break i;
        }
    };
    println!("{found:?}");
}
