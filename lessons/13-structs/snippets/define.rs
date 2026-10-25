struct Book {
    title: String,
    pages: u32,
    read: bool,
}

fn main() {
    // Fields can be given in any order.
    let mut book = Book {
        pages: 310,
        title: String::from("Dune"),
        read: false,
    };
    println!("title: {}", book.title);
    println!("pages: {}", book.pages);
    book.read = true;
    println!("read: {}", book.read);
}
