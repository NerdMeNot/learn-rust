struct Point {
    x: i32,
    y: i32,
}

// a parameter can be a pattern: this takes the point apart
fn manhattan(Point { x, y }: &Point) -> i32 {
    x.abs() + y.abs()
}

fn main() {
    let p = Point { x: 3, y: -4 };

    // let
    let Point { x, y } = p;
    println!("let: x = {x}, y = {y}");

    // function parameter
    println!("parameter: {}", manhattan(&p));

    // for: each item is split into a pair
    let scores = [("ana", 90), ("bo", 72)];
    for (name, score) in scores {
        println!("for: {name} scored {score}");
    }

    // if let: only when the shape fits
    if let Some((key, value)) = "mode=fast".split_once('=') {
        println!("if let: {key} -> {value}");
    }

    // while let: until the shape stops fitting
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("while let: popped {top}");
    }

    // let ... else: the shape must fit, or leave
    let Some((head, _)) = "a,b".split_once(',') else {
        return;
    };
    println!("let else: head = {head}");

    // a closure parameter
    let add = |(a, b): (i32, i32)| a + b;
    println!("closure: {}", add((2, 5)));

    // matches!
    println!("matches!: {}", matches!(p, Point { x: 3, .. }));
}
