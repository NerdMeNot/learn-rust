fn main() {
    let _scratch = 5; // never read: no warning
    let _ = 6; // binds nothing
    let _kept = 7; // still a real variable
    println!("kept: {_kept}");
}
