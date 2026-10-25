fn find_age(id: u32) -> Option<u32> {
    match id {
        1 => Some(31),
        2 => Some(15),
        _ => None,
    }
}

fn main() {
    for id in [1, 2, 3] {
        let age = find_age(id);
        println!(
            "id {id}: is_some {}, is_none {}, is_some_and(adult) {}",
            age.is_some(),
            age.is_none(),
            age.is_some_and(|a| a >= 18),
        );
    }

    // is_some_and takes the Option by value, like map. For a String, borrow first.
    let name: Option<String> = Some(String::from("ana"));
    println!("long name: {}", name.as_ref().is_some_and(|n| n.len() > 5));
    println!("name still there: {name:?}");
}
