fn main() {
    let name = String::from("Ada");
    let items: Vec<i32> = Vec::new();
    let found = Some(3);
    let count = 4;
    if name {
        println!("has a name");
    }
    if items {
        println!("has items");
    }
    if found {
        println!("found");
    }
    if !count {
        println!("zero");
    }
}
