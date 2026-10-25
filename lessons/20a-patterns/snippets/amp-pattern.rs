fn show<T>(label: &str, _value: &T) {
    println!("{label}: {}", std::any::type_name::<T>());
}

fn main() {
    let pairs = vec![(1, 10), (2, 20)];

    // `first` gives an Option holding a reference to the first pair
    // no &: the pattern passes through the reference, so a and b are references
    if let Some((a, b)) = pairs.first() {
        show("Some((a, b)), a", &a);
        show("Some((a, b)), b", &b);
    }
    // with &: the pattern strips the reference, so a and b are plain numbers
    if let Some(&(a, b)) = pairs.first() {
        show("Some(&(a, b)), a", &a);
        show("Some(&(a, b)), b", &b);
    }

    // in a loop it is the same choice
    let mut total = 0;
    for &(a, b) in &pairs {
        total += a * b;
    }
    println!("total: {total}");
}
