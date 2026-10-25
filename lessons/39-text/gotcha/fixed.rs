fn main() {
    let line = "ana,30,lisbon";

    // only need the first piece? no list needed
    let name = line.split(',').next().unwrap_or("");
    println!("{name}");

    // want to index around? collect into a Vec of views
    let parts: Vec<&str> = line.split(',').collect();
    println!("{} is {}", parts[0], parts[1]);
}
