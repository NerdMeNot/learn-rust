fn length(s: String) -> usize {
    s.len()
}

fn main() {
    let name = String::from("Ferris");
    let n = length(name);
    println!("{n} {name}");
}
