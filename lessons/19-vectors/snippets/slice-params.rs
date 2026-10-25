fn total(xs: &[i32]) -> i32 {
    let mut sum = 0;
    for x in xs {
        sum += x;
    }
    sum
}

fn double_all(xs: &mut [i32]) {
    for x in xs {
        *x *= 2;
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let arr = [10, 20, 30];

    println!("total(&v):        {}", total(&v));
    println!("total(&arr):      {}", total(&arr));
    println!("total(&v[1..3]):  {}", total(&v[1..3]));

    double_all(&mut v[3..]);
    println!("after double_all(&mut v[3..]): {v:?}");
    double_all(&mut v);
    println!("after double_all(&mut v):      {v:?}");
}
