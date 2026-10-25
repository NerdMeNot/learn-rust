use std::mem::size_of_val;

fn main() {
    let arr = [10, 20, 30, 40];
    let v = vec![10, 20, 30, 40];
    let from_arr: &[i32] = &arr[1..3];
    let from_vec: &[i32] = &v[1..3];

    println!(
        "arr:      {} bytes, the items themselves",
        size_of_val(&arr)
    );
    println!(
        "v:        {} bytes, a header; items: {} bytes",
        size_of_val(&v),
        size_of_val(&v[..])
    );
    println!(
        "from_arr: {} bytes, len {}",
        size_of_val(&from_arr),
        from_arr.len()
    );
    println!(
        "from_vec: {} bytes, len {}",
        size_of_val(&from_vec),
        from_vec.len()
    );
    println!(
        "from_arr points {} bytes into arr",
        from_arr.as_ptr() as usize - arr.as_ptr() as usize
    );
    println!(
        "from_vec points {} bytes into v's buffer",
        from_vec.as_ptr() as usize - v.as_ptr() as usize
    );
}
