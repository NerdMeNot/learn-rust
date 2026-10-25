use std::mem;

fn main() {
    let mut names = vec![
        String::from("ana"),
        String::from("bo"),
        String::from("cy"),
        String::from("di"),
    ];

    // 1. Copy it. names is untouched.
    let a = names[0].clone();
    println!("clone: {a:?}, names {names:?}");

    // 2. Take it out. remove shifts the rest down; swap_remove fills the gap
    // with the last item, which is faster.
    let b = names.remove(0);
    println!("remove(0): {b:?}, names {names:?}");
    let c = names.swap_remove(0);
    println!("swap_remove: {c:?}, names {names:?}");

    // 3. Take it and leave an empty value in its place.
    let d = mem::take(&mut names[0]);
    println!("mem::take: {d:?}, names {names:?}");

    // 4. Take it and leave a value you choose in its place.
    let e = mem::replace(&mut names[1], String::from("new"));
    println!("mem::replace: {e:?}, names {names:?}");

    // 5. Option::take: take what's inside and leave None.
    let mut slot = Some(String::from("token"));
    let f = slot.take();
    println!("Option::take: {f:?}, slot {slot:?}");

    // 6. Done with the whole Vec? Loop over it by value and own each item.
    for name in names {
        println!("owned: {name:?}");
    }
}
