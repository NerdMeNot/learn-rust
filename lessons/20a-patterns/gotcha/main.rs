fn main() {
    let line = "mode";
    let Some((key, value)) = line.split_once('=');
    println!("{key} -> {value}");
}
