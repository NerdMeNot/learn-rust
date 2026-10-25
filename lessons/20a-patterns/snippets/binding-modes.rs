fn show<T>(label: &str, _value: &T) {
    println!("{label}: {}", std::any::type_name::<T>());
}

fn main() {
    let mut n = Some(5);
    let pair = (1, 2);

    // the value itself: the name gets a copy
    if let Some(x) = n {
        show("if let Some(x) = n", &x);
    }
    // a shared reference to the value: the name gets a shared reference
    if let Some(x) = &n {
        show("if let Some(x) = &n", &x);
    }
    // a mutable reference: the name gets a mutable reference
    if let Some(x) = &mut n {
        *x += 1;
        show("if let Some(x) = &mut n", &x);
    }
    println!("n is now {n:?}");

    // the same for tuples
    let (a, _) = &pair;
    show("let (a, _) = &pair", &a);
    // a & in the pattern strips the reference by hand
    let &(b, _) = &pair;
    show("let &(b, _) = &pair", &b);
}
