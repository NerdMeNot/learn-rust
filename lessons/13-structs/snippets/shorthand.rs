#[derive(Debug)]
struct Book {
    title: String,
    pages: u32,
    read: bool,
}

fn new_book(title: String, pages: u32) -> Book {
    // `title` alone means `title: title`.
    Book {
        title,
        pages,
        read: false,
    }
}

fn main() {
    let book = new_book(String::from("Dune"), 310);
    println!("book: {book:?}");
    println!(
        "{} has {} pages, read: {}",
        book.title, book.pages, book.read
    );
}
