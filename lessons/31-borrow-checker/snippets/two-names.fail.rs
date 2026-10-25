struct Holder<'a> {
    list: &'a mut Vec<u32>,
}

fn main() {
    let mut scores = vec![1, 2, 3];
    let first = Holder { list: &mut scores };
    let second = Holder { list: &mut scores };
    first.list.push(4);
    second.list.push(5);
}
