fn main() {
    let names: Vec<String> = std::env::args().skip(1).collect();
    let done = names.len() > 2;
    if done == true {
        println!("done");
    }
    if names.len() == 0 {
        println!("no names");
    }
}
