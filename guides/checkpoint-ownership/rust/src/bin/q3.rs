fn bump(mut x: i32, mut arr: [i32; 3], list: &mut Vec<i32>) {
    x += 10;
    arr[0] += 10;
    list[0] += 10;
    println!("inside: {x} {arr:?} {list:?}");
}

fn main() {
    let x = 1;
    let arr = [1, 2, 3];
    let mut list = vec![1, 2, 3];
    bump(x, arr, &mut list);
    println!("outside: {x} {arr:?} {list:?}");
}
