#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let n = 6;
    let r = &n;
    println!("& as a reference: {r}");
    println!("* as dereference: {}, as multiply: {}", *r, *r * 2);
    let mut count = 1;
    let r_mut = &mut count;
    *r_mut += 1;
    println!("&mut then * changes the original: {count}");
    println!("& as bit-and: {}", 0b110 & 0b011);

    let or_bits = |a: u8, b: u8| a | b;
    println!("| in a closure, as bit-or: {}", or_bits(0b100, 0b001));
    match n {
        1 | 2 => println!("| in a pattern: one or two"),
        _ => println!("| in a pattern: neither"),
    }

    let p = Point { x: 1, y: 2 };
    let q = Point { x: 9, ..p };
    let [first, ..] = [10, 20, 30];
    println!(".. as a range: {:?}", 0..3);
    println!(".. in a struct: x={}, y={}", q.x, q.y);
    println!(".. in a pattern: {first}");

    let big = 1_000_000;
    let letters: Vec<_> = "abc".chars().collect();
    println!("_ in a number: {big}, as a type to fill in: {letters:?}");

    let done = false;
    println!("! as not: {}, but also in println!", !done);
    println!("'a' is a char: {}", 'a');
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i + j == 3 {
                println!("' as a loop label: left both loops at {i},{j}");
                break 'outer;
            }
        }
    }
    println!("< as less-than: {}", 1 < 2);
    println!("::<> picks a type: {:?}", "5".parse::<i32>());
}
