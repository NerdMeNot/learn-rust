use std::collections::BTreeMap;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use serde_json::{Value, json};
use shop_core::{Catalog, Error, Money, Product, Sku};

/// A catalog kept in a file, one JSON object per line. Every call reads or
/// rewrites the whole file, which is fine for a small shop and no more.
#[derive(Debug)]
pub struct FileCatalog {
    path: PathBuf,
}

impl FileCatalog {
    /// A catalog stored at `path`. The file is created by the first `put`.
    pub fn new(path: impl Into<PathBuf>) -> FileCatalog {
        FileCatalog { path: path.into() }
    }

    fn load(&self) -> Result<BTreeMap<Sku, Product>, Error> {
        let text = match fs::read_to_string(&self.path) {
            Ok(text) => text,
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(BTreeMap::new()),
            Err(e) => return Err(Error::Store(e.to_string())),
        };
        let mut items = BTreeMap::new();
        for (n, line) in text.lines().enumerate() {
            let product = parse_line(line)
                .ok_or_else(|| Error::Store(format!("line {} is not a product", n + 1)))?;
            items.insert(product.sku.clone(), product);
        }
        Ok(items)
    }
}

fn parse_line(line: &str) -> Option<Product> {
    let value: Value = serde_json::from_str(line).ok()?;
    Some(Product {
        sku: value["sku"].as_str()?.parse().ok()?,
        name: value["name"].as_str()?.to_string(),
        price: Money::from_cents(value["cents"].as_u64()?),
    })
}

impl Catalog for FileCatalog {
    fn get(&self, sku: &Sku) -> Result<Option<Product>, Error> {
        Ok(self.load()?.remove(sku))
    }

    fn put(&self, product: Product) -> Result<(), Error> {
        let mut items = self.load()?;
        items.insert(product.sku.clone(), product);
        let mut text = String::new();
        for p in items.values() {
            let line = json!({ "sku": p.sku.as_str(), "name": p.name, "cents": p.price.cents() });
            text.push_str(&line.to_string());
            text.push('\n');
        }
        fs::write(&self.path, text).map_err(|e| Error::Store(e.to_string()))
    }

    fn list(&self) -> Result<Vec<Product>, Error> {
        Ok(self.load()?.into_values().collect())
    }
}
