//! `shop [--db FILE] [REQUEST...]`: runs the request given on the command line,
//! or one request per line from standard input.

use std::io::{self, BufRead};
use std::process::ExitCode;

use shop_api::Api;
use shop_core::Catalog;
use shop_storage::MemoryCatalog;

#[cfg(feature = "file")]
fn open(path: &str) -> Result<Box<dyn Catalog>, String> {
    Ok(Box::new(shop_storage::FileCatalog::new(path)))
}

#[cfg(not(feature = "file"))]
fn open(_path: &str) -> Result<Box<dyn Catalog>, String> {
    Err("--db needs the `file` feature: cargo run -p shop-cli --features file".to_string())
}

fn main() -> ExitCode {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let catalog: Box<dyn Catalog> = if args.first().map(String::as_str) == Some("--db") {
        if args.len() < 2 {
            eprintln!("shop: --db needs a file name");
            return ExitCode::from(2);
        }
        let path = args[1].clone();
        args.drain(..2);
        match open(&path) {
            Ok(catalog) => catalog,
            Err(why) => {
                eprintln!("shop: {why}");
                return ExitCode::from(2);
            }
        }
    } else {
        Box::new(MemoryCatalog::new())
    };
    let api = Api::new(catalog);

    let requests: Vec<String> = if args.is_empty() {
        io::stdin().lock().lines().map_while(Result::ok).filter(|l| !l.trim().is_empty()).collect()
    } else {
        vec![args.join(" ")]
    };

    let mut failed = false;
    for request in requests {
        let response = api.handle(&request);
        println!("{} {}", response.status, response.body);
        failed |= response.status >= 400;
    }
    if failed { ExitCode::from(1) } else { ExitCode::SUCCESS }
}
