fn main() {
    let word = String::from("naïve café");
    println!("{} {}", word.len(), word.chars().count());
    println!("{:?}", word.find('v'));
    println!("{:?}", word.find("é"));
    println!("{}", &word[1..4]);
}
