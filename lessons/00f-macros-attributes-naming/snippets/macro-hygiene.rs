macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

macro_rules! double_into {
    ($dst:ident, $src:expr) => {
        let tmp = $src * 2;
        $dst = tmp;
    };
}

fn square_fn(x: i32) -> i32 {
    x * x
}

fn main() {
    println!("square!(1 + 2) = {}", square!(1 + 2));
    let mut tmp = 0;
    println!("tmp before: {tmp}");
    double_into!(tmp, 5);
    println!("tmp after double_into!(tmp, 5) = {tmp}");

    let mut i = 0;
    let r = square!({
        i += 1;
        i
    });
    println!("square!({{ i += 1; i }}) = {r}, and i = {i}");
    let mut j = 0;
    let r = square_fn({
        j += 1;
        j
    });
    println!("square_fn(..) = {r}, and j = {j}");
}
