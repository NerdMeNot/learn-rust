fn main() {
    let mut found = 0;
    'outer: for i in 1..=4 {
        for j in i..=4 {
            if (i * j) % 3 == 0 {
                found += 1;
                continue 'outer;
            }
            if i + j > 6 {
                break 'outer;
            }
            println!("{i},{j}");
        }
    }
    println!("found {found}");
}
