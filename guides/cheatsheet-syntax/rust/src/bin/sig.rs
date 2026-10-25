use std::fmt::Display;

// pub fn  name  <generics>  (parameters)  -> result  where bounds
pub fn longest_by<'a, T, K>(items: &'a [T], key: impl Fn(&T) -> K) -> Option<&'a T>
where
    K: Ord + Display,
{
    let mut best: Option<&T> = None;
    for item in items {
        match best {
            Some(b) if key(b) >= key(item) => {}
            _ => best = Some(item),
        }
    }
    best
}

fn main() {
    let words = ["fig", "banana", "kiwi"];
    println!("{:?}", longest_by(&words, |w| w.len()));
    let none: [&str; 0] = [];
    println!("{:?}", longest_by(&none, |w| w.len()));
}
