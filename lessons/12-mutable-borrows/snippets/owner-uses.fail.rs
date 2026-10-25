fn assign_while_read() {
    let mut n = 10;
    let reader = &n;
    n = 20; // the owner assigns while a reader is alive
    println!("reader sees {reader}");
}

fn assign_while_written() {
    let mut n = 10;
    let writer = &mut n;
    n = 20; // the owner assigns while a writer is alive
    *writer += 1;
}

fn read_while_written() {
    let mut n = 10;
    let writer = &mut n;
    let copy = n; // the owner reads while a writer is alive
    *writer += 1;
    println!("copy: {copy}");
}

fn main() {
    assign_while_read();
    assign_while_written();
    read_while_written();
}
