fn main() {
    for text in ["1_000", " 12 "] {
        println!("{text:?}.parse::<i32>(): {:?}", text.parse::<i32>());
    }

    let xs = vec![1, 2, 3, 4];
    println!("xs.get(1..10): {:?}", xs.get(1..10));

    let mut words = vec!["éé", "abc"];
    words.sort_by_key(|s| s.len());
    println!("sort_by_key(len): {words:?}");

    println!("xs[1..10]: {:?}", &xs[1..10]);
}
