struct Book {
    title: String,
    read: bool,
}

fn main() {
    let book = Book {
        title: String::from("Dune"),
        read: false,
    };
    book.read = true;
    println!("{} read: {}", book.title, book.read);
}
