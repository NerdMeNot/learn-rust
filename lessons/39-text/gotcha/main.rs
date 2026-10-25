fn main() {
    let line = "ana,30,lisbon";
    let parts = line.split(',');
    let name = parts[0];
    println!("{name}");
}
