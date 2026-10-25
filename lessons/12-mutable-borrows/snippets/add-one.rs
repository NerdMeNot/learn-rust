fn add_one(x: &mut i32) {
    *x += 1;
    println!("inside add_one: x = {x}");
}

fn add_one_to_copy(mut x: i32) -> i32 {
    x += 1;
    x
}

fn main() {
    let mut n = 5;
    add_one(&mut n);
    println!("n after add_one: {n}");

    let copy_result = add_one_to_copy(n);
    println!("add_one_to_copy returned {copy_result}, n is still {n}");
}
