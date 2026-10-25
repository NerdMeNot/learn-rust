//! One set of checks that every `Catalog` must pass, run against each backend.

use shop_core::{Catalog, Money, Product};
use shop_storage::MemoryCatalog;

fn product(sku: &str, name: &str, cents: u64) -> Product {
    Product { sku: sku.parse().unwrap(), name: name.to_string(), price: Money::from_cents(cents) }
}

fn contract(catalog: &dyn Catalog) {
    assert!(catalog.list().unwrap().is_empty());
    assert_eq!(catalog.get(&"TEA".parse().unwrap()).unwrap(), None);

    catalog.put(product("TEA", "Green tea, 100 g", 450)).unwrap();
    catalog.put(product("MUG", "Enamel mug", 1200)).unwrap();
    catalog.put(product("TEA", "Green tea, 200 g", 800)).unwrap();

    let all = catalog.list().unwrap();
    let skus: Vec<&str> = all.iter().map(|p| p.sku.as_str()).collect();
    assert_eq!(skus, ["MUG", "TEA"], "listed in SKU order, TEA stored once");
    assert_eq!(
        catalog.get(&"TEA".parse().unwrap()).unwrap(),
        Some(product("TEA", "Green tea, 200 g", 800))
    );
}

#[test]
fn memory_catalog_keeps_the_contract() {
    contract(&MemoryCatalog::new());
}

#[cfg(feature = "file")]
#[test]
fn file_catalog_keeps_the_contract() {
    use shop_storage::FileCatalog;

    let path = std::env::temp_dir().join(format!("shop-contract-{}.jsonl", std::process::id()));
    let _ = std::fs::remove_file(&path);
    contract(&FileCatalog::new(&path));
    // A second catalog on the same file sees what the first one wrote.
    assert_eq!(FileCatalog::new(&path).list().unwrap().len(), 2);
    std::fs::remove_file(&path).unwrap();
}
