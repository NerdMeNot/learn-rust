fn main() {
    let v = vec![1, 2, 3];
    let chain = v
        .iter()
        .map(|x| {
            println!("map {x}");
            x * 2
        })
        .filter(|x| {
            println!("filter {x}");
            *x > 2
        });
    println!("chain built");
    let total: i32 = chain.sum();
    println!("total: {total}");
}
