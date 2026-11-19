fn main() {
    let mut names = vec![String::from("ada"), String::from("grace")];
    // An owned copy of one name: the borrow of `names` ends on this line.
    let longest = names.iter().max_by_key(|n| n.len()).unwrap().clone();
    names.push(String::from("barbara"));
    println!("longest: {longest}, names: {names:?}");
}
