fn main() {
    let mut names = vec![String::from("ada"), String::from("grace")];
    let longest = names.iter().max_by_key(|n| n.len()).unwrap();
    names.push(String::from("barbara"));
    println!("longest: {longest}");
}
