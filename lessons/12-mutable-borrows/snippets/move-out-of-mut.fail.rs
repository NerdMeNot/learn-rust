fn take_name(name: &mut String) -> String {
    *name
}

fn main() {
    let mut name = String::from("Ada");
    let taken = take_name(&mut name);
    println!("taken: {taken}");
}
