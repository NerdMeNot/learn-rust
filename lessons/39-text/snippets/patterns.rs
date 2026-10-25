fn main() {
    let text = "ab1,cd22;ef";

    // The same method with five kinds of pattern.
    println!("char:       {:?}", text.split(',').collect::<Vec<_>>());
    println!("&str:       {:?}", text.split(",c").collect::<Vec<_>>());
    println!(
        "char array: {:?}",
        text.split([',', ';']).collect::<Vec<_>>()
    );
    println!(
        "closure:    {:?}",
        text.split(|c: char| c.is_ascii_digit()).collect::<Vec<_>>()
    );
    println!(
        "fn item:    {:?}",
        text.split(char::is_numeric).collect::<Vec<_>>()
    );

    // Every method that searches takes the same kinds.
    println!("find a digit:   {:?}", text.find(char::is_numeric));
    println!("rfind , or ;:   {:?}", text.rfind([',', ';']));
    println!("digits counted: {}", text.matches(char::is_numeric).count());
    println!("trim_matches:   {:?}", ",;ab,;".trim_matches([',', ';']));
    println!(
        "starts with a letter: {}",
        text.starts_with(|c: char| c.is_alphabetic())
    );
}
