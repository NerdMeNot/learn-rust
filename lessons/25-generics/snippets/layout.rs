use std::mem::size_of;

struct Pair<T> {
    first: T,
    second: T,
}

fn main() {
    println!("size_of::<i32>(): {}", size_of::<i32>());
    println!("size_of::<Pair<i32>>(): {}", size_of::<Pair<i32>>());
    println!("size_of::<String>(): {}", size_of::<String>());
    println!("size_of::<Pair<String>>(): {}", size_of::<Pair<String>>());

    let numbers = Pair {
        first: 1,
        second: 2,
    };
    let names = Pair {
        first: String::from("ana"),
        second: String::from("bo"),
    };
    println!("numbers: {} and {}", numbers.first, numbers.second);
    println!("names: {} and {}", names.first, names.second);
    println!(
        "first name: len {}, capacity {}",
        names.first.len(),
        names.first.capacity()
    );
    println!(
        "second name: len {}, capacity {}",
        names.second.len(),
        names.second.capacity()
    );
}
