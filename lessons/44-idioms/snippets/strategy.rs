use std::fmt::Display;

/// The rule is a parameter. Any closure (or function) with this shape fits.
fn count_matching(items: &[i32], keep: impl Fn(i32) -> bool) -> usize {
    let mut n = 0;
    for &item in items {
        if keep(item) {
            n += 1;
        }
    }
    n
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// Chosen at run time (say, from a config file): a boxed closure, one type for every rule.
fn rule(name: &str) -> Option<Box<dyn Fn(i32) -> bool>> {
    match name {
        "even" => Some(Box::new(is_even)),
        "big" => Some(Box::new(|n| n > 5)),
        _ => None,
    }
}

/// `impl Trait` for the item type too: anything that can be iterated and shown.
fn show_all(items: impl IntoIterator<Item = impl Display>) -> String {
    let parts: Vec<String> = items.into_iter().map(|i| i.to_string()).collect();
    parts.join(" | ")
}

fn main() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let limit = 5;
    println!("even:       {}", count_matching(&data, is_even));
    println!("above {limit}:    {}", count_matching(&data, |n| n > limit));
    println!(
        "odd or > 6: {}",
        count_matching(&data, |n| n % 2 == 1 || n > 6)
    );

    for name in ["big", "even", "prime"] {
        match rule(name) {
            Some(keep) => println!("rule {name}: {}", count_matching(&data, keep)),
            None => println!("rule {name}: unknown"),
        }
    }

    println!("{}", show_all(vec![1, 2, 3]));
    println!("{}", show_all(["a", "b"]));
    println!("{}", show_all(data.iter().take(2)));
}
