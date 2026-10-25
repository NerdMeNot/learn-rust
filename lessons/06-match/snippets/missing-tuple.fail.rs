fn main() {
    let (raining, windy) = (true, false);
    let advice = match (raining, windy) {
        (true, true) => "stay in",
        (true, false) => "umbrella",
        (false, true) => "jacket",
    };
    println!("{advice}");
}
