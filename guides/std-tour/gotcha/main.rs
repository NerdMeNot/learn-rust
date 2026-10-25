fn main() {
    let names = vec!["ana", "bo"]; // Vec: always there
    let mut ages = HashMap::new(); // HashMap: not imported
    ages.insert("ana", 30);
    println!("{} {}", names.len(), ages.len());
}
