fn by_value(mut n: i32) -> i32 {
    n += 1;
    n
}

fn by_ref(n: &i32) -> i32 {
    *n + 1
}

fn by_mut(n: &mut i32) {
    *n += 1;
}

fn main() {
    let mut a = 5;

    println!("by_value(a) returns:  {}", by_value(a));
    println!("after by_value(a):    a = {a}");
    println!("by_ref(&a) returns:   {}", by_ref(&a));
    println!("after by_ref(&a):     a = {a}");
    by_mut(&mut a);
    println!("after by_mut(&mut a): a = {a}");

    let b = a; // numbers are Copy: b is its own number
    a += 100;
    println!("after let b = a:      a = {a}, b = {b}");

    let s = String::from("hi");
    let mut t = s.clone(); // an explicit, independent copy
    t.push('!');
    println!("after s.clone():      s = {s}, t = {t}");
}
