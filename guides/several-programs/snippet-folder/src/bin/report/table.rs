pub fn print(rows: &[(&str, u32)]) {
    for (name, n) in rows {
        println!("{name:<8}{n}");
    }
}
