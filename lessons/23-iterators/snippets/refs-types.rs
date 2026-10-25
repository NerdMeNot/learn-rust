use std::any::type_name_of_val as type_of;

fn main() {
    let nums = vec![10, 20, 30];

    // Each line asks the compiler for the real type of what the closure receives.
    let first = nums.iter().next().unwrap();
    println!("iter() hands out           {}", type_of(&first));

    let _ = nums
        .iter()
        .map(|n| println!("map's closure gets         {}", type_of(&n)))
        .next();
    let _ = nums
        .iter()
        .filter(|n| {
            println!("filter's closure gets      {}", type_of(&n));
            true
        })
        .next();
    let _ = nums
        .iter()
        .copied()
        .filter(|n| {
            println!("after copied(), filter gets {}", type_of(&n));
            true
        })
        .next();
    let _ = nums
        .clone()
        .into_iter()
        .map(|n| println!("into_iter(), map gets      {}", type_of(&n)))
        .next();
}
