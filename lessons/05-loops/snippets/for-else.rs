fn first_negative(xs: &[i32]) -> &str {
    // a labelled block gives a value, like Python's for/else
    'search: {
        // `&x` takes the number out of the reference
        for &x in xs {
            if x < 0 {
                break 'search "found a negative";
            }
        }
        "none"
    }
}

fn main() {
    println!("[3, -1, 4]: {}", first_negative(&[3, -1, 4]));
    println!("[3, 1, 4]: {}", first_negative(&[3, 1, 4]));
}
