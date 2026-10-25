fn main() {
    println!("{}", "tab:\t quote:\" newline:\n end");
    println!("{:?}", "tab:\t quote:\" newline:\n end");
    println!("{:?} {:?} {:?}", 'a', '\'', '\u{e9}');
    println!("{:?}", "it's");
    println!("{:?}", "e\u{301} \u{200b} \u{7f}");
    println!("{:?} {:?}", "日本", String::from("smile \u{1F600}"));
    println!("{} {:?}", 1.0, 1.0);
    println!("{} {:?}", 1e21, 1e21);
}
