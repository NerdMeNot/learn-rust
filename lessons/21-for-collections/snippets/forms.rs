use std::any::type_name_of_val;

fn main() {
    let mut nums = vec![10, 20];

    for x in &nums {
        println!("for x in &nums: x is {} ({x})", type_name_of_val(&x));
    }

    for x in &mut nums {
        *x += 1;
        println!("for x in &mut nums: x is {} ({x})", type_name_of_val(&x));
    }

    for x in nums {
        println!("for x in nums: x is {} ({x})", type_name_of_val(&x));
    }
}
