//! Uses the file-backed catalog without asking for the `file` feature.
use shop_storage::FileCatalog;

fn main() {
    let _catalog = FileCatalog::new("shop.jsonl");
}
