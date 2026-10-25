use std::mem::{align_of, offset_of, size_of};

struct User {
    name: String,
    age: u32,
    active: bool,
}

fn main() {
    let u = User {
        name: String::from("ana"),
        age: 30,
        active: true,
    };

    println!(
        "User: size {}, align {}",
        size_of::<User>(),
        align_of::<User>()
    );
    println!("String: size {}", size_of::<String>());
    println!(
        "offsets: name {}, age {}, active {}",
        offset_of!(User, name),
        offset_of!(User, age),
        offset_of!(User, active)
    );
    println!(
        "name: len {}, capacity {}, bytes {:?}",
        u.name.len(),
        u.name.capacity(),
        u.name.as_bytes()
    );
    println!("age: {}, active: {}", u.age, u.active);
}
