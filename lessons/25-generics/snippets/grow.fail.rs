// N is a value inside the body, but a type may use it only as a bare `N`.
fn grow<const N: usize>(items: [i32; N]) -> [i32; N + 1] {
    todo!("{items:?}")
}

fn main() {
    println!("{:?}", grow([1, 2]));
}
