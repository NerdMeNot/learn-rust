fn main() {
    let mut v: Vec<u32> = Vec::new();
    println!("start: len {}, capacity {}", v.len(), v.capacity());

    let mut last_ptr = v.as_ptr();
    let mut last_cap = v.capacity();
    for i in 1..=40 {
        v.push(i);
        if v.capacity() != last_cap {
            let what = if last_cap == 0 {
                "first buffer"
            } else if v.as_ptr() != last_ptr {
                "buffer moved"
            } else {
                "same address"
            };
            println!("push #{i}: capacity {last_cap} -> {}, {what}", v.capacity());
            last_cap = v.capacity();
            last_ptr = v.as_ptr();
        }
    }
    println!("end: len {}, capacity {}", v.len(), v.capacity());

    let mut w: Vec<u32> = Vec::with_capacity(40);
    let before = w.as_ptr();
    for i in 1..=40 {
        w.push(i);
    }
    println!(
        "with_capacity(40): after 40 pushes capacity {}, buffer moved: {}",
        w.capacity(),
        w.as_ptr() != before
    );

    w.truncate(3);
    println!(
        "after truncate(3): len {}, capacity {}",
        w.len(),
        w.capacity()
    );
    w.shrink_to_fit();
    println!(
        "after shrink_to_fit: len {}, capacity {}",
        w.len(),
        w.capacity()
    );

    let mut r = vec![1, 2, 3];
    println!("vec![1, 2, 3]: capacity {}", r.capacity());
    r.reserve(5);
    println!("after reserve(5): capacity {}", r.capacity());
}
