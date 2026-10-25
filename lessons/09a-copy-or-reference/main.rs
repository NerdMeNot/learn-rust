fn main() {
    let mut ages = vec![30, 40, 50];

    // A copy: `age` is its own number, so the Vec does not change.
    for mut age in ages.iter().copied() {
        age += 1;
        println!("the copy is now {age}");
    }
    println!("ages after the copy loop:     {ages:?}");

    // The elements themselves: `&mut` in the code says so.
    for age in &mut ages {
        *age += 1;
    }
    println!("ages after the in-place loop: {ages:?}");
}
