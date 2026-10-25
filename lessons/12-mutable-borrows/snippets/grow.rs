fn push_and_report(label: &str, mut v: Vec<i32>) {
    let before = v.as_ptr();
    let capacity_before = v.capacity();
    v.push(5);
    println!(
        "{label} capacity {capacity_before} -> {}, buffer moved: {}",
        v.capacity(),
        v.as_ptr() != before
    );
}

fn main() {
    let alone = vec![1, 2, 3, 4];
    push_and_report("no neighbour:", alone);

    let crowded = vec![1, 2, 3, 4];
    let _neighbour = vec![0u8; 16]; // another allocation, probably right after `crowded`'s
    push_and_report("a neighbour: ", crowded);
}
