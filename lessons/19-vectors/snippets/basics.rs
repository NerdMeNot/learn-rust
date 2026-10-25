fn main() {
    let mut v = vec![10, 20, 30];

    v.push(40);
    println!("after push(40): {v:?}");
    println!("pop(): {:?}", v.pop());
    println!("len: {}, is_empty: {}", v.len(), v.is_empty());

    println!("v[1]: {}", v[1]);
    v[1] = 25;
    println!("after v[1] = 25: {v:?}");

    println!("get(1): {:?}", v.get(1));
    println!("get(10): {:?}", v.get(10));
    println!("first(): {:?}, last(): {:?}", v.first(), v.last());

    let mut empty: Vec<i32> = Vec::new();
    println!(
        "empty.pop(): {:?}, empty.last(): {:?}",
        empty.pop(),
        empty.last()
    );
}
