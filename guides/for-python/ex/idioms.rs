use std::fmt::Debug;
use std::fs;

fn show<T: Debug>(label: &str, value: T) {
    println!("{label}: {value:?}");
}

struct Guard;

impl Guard {
    fn new() -> Guard {
        println!("open");
        Guard
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        println!("close");
    }
}

fn port_or_default(text: &str) -> i32 {
    text.parse::<i32>().unwrap_or(-1)
}

fn main() {
    let nums = [1, 2, 3, 4];
    let words = vec!["pear", "fig", "apple"];
    let text = "a,b,c";
    let s = "héllo";
    let xs = [1, 2, 3, 4];
    let m = std::collections::HashMap::from([("a", 1)]);
    let x: Option<i32> = None;
    let items = vec![7];
    fs::write("note.txt", "hello\n").unwrap();

    show("map", nums.iter().map(|n| n * n).collect::<Vec<_>>());
    show(
        "filter",
        nums.iter().filter(|n| *n % 2 == 0).collect::<Vec<_>>(),
    );
    show("sorted by length", {
        let mut w = words.clone();
        w.sort_by_key(|s| s.len());
        w
    });
    show(
        "enumerate",
        words
            .iter()
            .enumerate()
            .map(|(i, w)| format!("{i}:{w}"))
            .collect::<Vec<_>>(),
    );
    show("contains", text.contains("b,"));
    show("split", text.split(',').collect::<Vec<_>>());
    show("join", words.join(", "));
    show("chars", s.chars().count());
    show("bytes", s.len());
    show("slice", xs[1..3].to_vec());
    show("get default", m.get("c").copied().unwrap_or(0));
    show("none default", x.unwrap_or(5));
    show("parse", port_or_default("4x"));
    show("floor division", (-7_i32).div_euclid(2));
    show("big int", 2_u128.pow(100));
    show("truthy list", !items.is_empty());
    show("file", fs::read_to_string("note.txt").unwrap());
    show("format", format!("{:>5}", 42));
    {
        let _guard = Guard::new();
        println!("work");
    }
}
