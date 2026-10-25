use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard};

use shop_core::{Catalog, Error, Product, Sku};

/// A catalog that lives as long as the program.
#[derive(Debug, Default)]
pub struct MemoryCatalog {
    items: Mutex<BTreeMap<Sku, Product>>,
}

impl MemoryCatalog {
    /// An empty catalog.
    pub fn new() -> MemoryCatalog {
        MemoryCatalog::default()
    }

    fn items(&self) -> Result<MutexGuard<'_, BTreeMap<Sku, Product>>, Error> {
        self.items.lock().map_err(|_| Error::Store("lock poisoned".into()))
    }
}

impl Catalog for MemoryCatalog {
    fn get(&self, sku: &Sku) -> Result<Option<Product>, Error> {
        Ok(self.items()?.get(sku).cloned())
    }

    fn put(&self, product: Product) -> Result<(), Error> {
        self.items()?.insert(product.sku.clone(), product);
        Ok(())
    }

    fn list(&self) -> Result<Vec<Product>, Error> {
        Ok(self.items()?.values().cloned().collect())
    }
}
