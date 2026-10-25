fn first_even(nums: &[i32]) -> Option<i32> {
    let check = |n: i32| {
        if n % 2 == 0 {
            return true; // leaves the closure, not first_even
        }
        false
    };
    for &n in nums {
        if check(n) {
            return Some(n);
        }
    }
    None
}

fn main() {
    println!("first_even([3, 5, 8, 9]): {:?}", first_even(&[3, 5, 8, 9]));
    println!("first_even([3, 5]): {:?}", first_even(&[3, 5]));
}
