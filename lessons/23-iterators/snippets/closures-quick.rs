fn main() {
    // A closure is a small function you write inline: |parameters| body.
    let double = |x: i32| x * 2;
    println!("double(21): {}", double(21));

    // Types are usually inferred, and a body can be a block.
    let describe = |n| {
        if n % 2 == 0 { "even" } else { "odd" }
    };
    println!("describe(7): {}", describe(7));

    // Unlike a plain function, a closure can use variables around it.
    let limit = 10;
    let over = |x: i32| x > limit;
    println!("over(12): {}", over(12));

    // Iterator methods take closures: "do this to each item".
    let tripled: Vec<i32> = vec![1, 2, 3].into_iter().map(|x| x * 3).collect();
    println!("tripled: {tripled:?}");
}
