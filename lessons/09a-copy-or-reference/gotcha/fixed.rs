fn main() {
    let mut ages = vec![30, 40, 50];
    for age in &mut ages {
        *age += 1; // `age` is a `&mut i32`: say `*` to reach the number
    }
    println!("{ages:?}");
}
