fn coalesce(a: Option<i32>) -> i32 {
    a ?? 5
}

fn strict(a: i32) -> bool {
    a === 1
}

fn each(v: Vec<i32>) {
    for x of v {
        println!("{x}");
    }
}

fn main() {}
