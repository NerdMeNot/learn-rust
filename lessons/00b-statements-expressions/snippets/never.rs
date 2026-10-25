fn main() {
    let mut sum = 0;
    for i in 0..6 {
        let even = if i % 2 == 0 { i } else { continue };
        sum += even;
    }
    println!("sum of evens: {sum}");

    let ok = true;
    let x: i32 = if ok { 5 } else { panic!("not ok") };
    println!("x: {x}");
}
