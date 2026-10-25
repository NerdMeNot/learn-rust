fn main() {
    let names = ["ada", "grace", "alan"];
    let width = names.iter().map(|n| n.len()).max().unwrap(); // the widest name
    for (i, name) in names.iter().enumerate() {
        let score = 2.0 * i as f64 + 0.3;
        println!("{i:>2} | {name:<width$} | {score:>6.1}");
    }

    let (w, p, x) = (9, 2, 3.14159);
    println!("[{x:>w$.p$}]"); // width and precision from variables
    println!("[{:>1$}]", 42, 6); // width is argument 1
    println!("[{:.*}]", 3, 2.71828); // .* takes the precision first, then the value
    println!("[{:>width$}]", 9, width = 4); // a named argument
}
