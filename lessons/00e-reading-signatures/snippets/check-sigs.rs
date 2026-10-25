// Backs the decoded std signatures on the page: each line gives the real std item
// the type the page says it has, so a wrong reading will not compile.
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::thread;

fn takes_ord<T: Ord>(_: T) {}

// one input reference: the compiler fills in the lifetime, so no `'a` is written
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}
fn takes_send_static<T: Send + 'static>(_: T) {}

fn main() {
    // cmp::max<T>(v1: T, v2: T) -> T where T: Ord
    let max_i32: fn(i32, i32) -> i32 = std::cmp::max;
    assert_eq!(max_i32(3, 9), 9);
    assert_eq!(std::cmp::max(1, 1), 1);
    assert_eq!(std::cmp::max("b", "a"), "b");
    takes_ord(1u8);

    // Option::map takes self by value, a FnOnce(T) -> U, and gives Option<U>
    let map_fn: fn(Option<i32>, fn(i32) -> String) -> Option<String> = Option::map;
    assert_eq!(map_fn(Some(2), |n| n.to_string()), Some("2".to_string()));
    let owned = String::from("moved in");
    let once = move |n: i32| format!("{owned}{n}"); // FnOnce is enough: may be called once
    assert_eq!(Some(1).map(once), Some("moved in1".to_string()));

    // HashMap::entry(&mut self, key: K) -> Entry<'_, K, V>
    let entry_fn: for<'a> fn(&'a mut HashMap<String, i32>, String) -> Entry<'a, String, i32> =
        HashMap::entry;
    let mut counts: HashMap<String, i32> = HashMap::new();
    *entry_fn(&mut counts, "a".to_string()).or_insert(0) += 1;
    *counts.entry("a".to_string()).or_insert(0) += 1;
    assert_eq!(counts["a"], 2);

    // thread::spawn<F, T>(f: F) -> JoinHandle<T>, F: FnOnce() -> T + Send + 'static, T: Send + 'static
    let handle: thread::JoinHandle<i32> = thread::spawn(|| 5);
    assert_eq!(handle.join().unwrap(), 5);
    takes_send_static(String::new());

    // fs::read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String>
    let read: fn(&'static str) -> std::io::Result<String> = std::fs::read_to_string::<&'static str>;
    assert!(read("/definitely/not/here").is_err());
    assert!(std::fs::read_to_string(std::path::Path::new("/nope")).is_err());
    assert!(std::fs::read_to_string(String::from("/nope")).is_err());

    // Vec::push(&mut self, value: T), slice::first(&self) -> Option<&T>
    let push: fn(&mut Vec<i32>, i32) = Vec::push;
    let mut v = Vec::new();
    push(&mut v, 4);
    let first: for<'a> fn(&'a [i32]) -> Option<&'a i32> = <[i32]>::first;
    assert_eq!(first(&v), Some(&4));
    assert_eq!(first(&[]), None);

    // process::exit(code: i32) -> !
    let _exit: fn(i32) -> ! = std::process::exit;

    // array::from_fn<T, const N: usize, F>(f: F) -> [T; N]
    let squares: [usize; 4] = std::array::from_fn(|i| i * i);
    assert_eq!(squares, [0, 1, 4, 9]);

    // Iterator::map<B, F>(self, f: F) -> Map<Self, F>, F: FnMut(Self::Item) -> B
    let mut calls = 0;
    let doubled: Vec<i32> = [1, 2, 3]
        .into_iter()
        .map(|x| {
            calls += 1; // FnMut: may change what it captured
            x * 2
        })
        .collect();
    assert_eq!((doubled, calls), (vec![2, 4, 6], 3));

    // Iterator::next(&mut self) -> Option<Self::Item>
    let mut it = [7].into_iter();
    assert_eq!((it.next(), it.next()), (Some(7), None));

    assert_eq!(first_word("hello world"), "hello");
    println!("signatures check out");
}
