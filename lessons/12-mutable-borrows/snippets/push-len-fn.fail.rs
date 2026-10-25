fn add(v: &mut Vec<usize>, n: usize) {
    v.push(n);
}

fn bump(v: &mut Vec<usize>) -> usize {
    v[0] += 1;
    v[0]
}

fn explicit_borrow_first() {
    let mut v = vec![1];
    add(&mut v, v.len()); // the `&mut v` you write is taken first
    println!("v: {v:?}");
}

fn argument_changes_the_list() {
    let mut v = vec![1];
    v.push(bump(&mut v)); // an argument that changes `v` is not just a read
    println!("v: {v:?}");
}

fn main() {
    explicit_borrow_first();
    argument_changes_the_list();
}
