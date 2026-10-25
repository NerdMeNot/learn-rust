fn power() -> i32 {
    2 ** 10
}

fn between(x: i32) -> bool {
    1 < x < 5
}

fn member(v: Vec<i32>) -> bool {
    2 in v
}

fn identity(x: Option<i32>) -> bool {
    x is None
}

fn flag() -> bool {
    True
}

fn main() {}
