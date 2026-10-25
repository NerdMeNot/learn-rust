fn main() {
    let line = "mode";
    let Some((key, value)) = line.split_once('=') else {
        println!("no '=' in {line:?}");
        return;
    };
    println!("{key} -> {value}");
}
