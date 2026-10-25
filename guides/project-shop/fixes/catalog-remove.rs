use crate::{Error, Money, Product, Sku};

/// Where products are kept. `shop-core` only says what a catalog can do;
/// the `shop-storage` crate holds the implementations.
pub trait Catalog: Send + Sync {
    /// The product with this SKU, if there is one.
    fn get(&self, sku: &Sku) -> Result<Option<Product>, Error>;

    /// Adds a product, or replaces the one with the same SKU.
    fn put(&self, product: Product) -> Result<(), Error>;

    /// Removes a product. Returns whether there was one to remove.
    fn remove(&self, sku: &Sku) -> Result<bool, Error>;

    /// Every product, in SKU order.
    fn list(&self) -> Result<Vec<Product>, Error>;
}

/// The price of an order: each `(sku, quantity)` line at its catalog price.
pub fn order_total(catalog: &dyn Catalog, lines: &[(Sku, u32)]) -> Result<Money, Error> {
    let mut total = Money::default();
    for (sku, quantity) in lines {
        let product = catalog.get(sku)?.ok_or_else(|| Error::NotFound(sku.clone()))?;
        let line = product.price.checked_mul(*quantity).ok_or(Error::Overflow)?;
        total = total.checked_add(line).ok_or(Error::Overflow)?;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A catalog made of a fixed list: enough to test `order_total` with no storage crate.
    struct Fixed(Vec<Product>);

    impl Catalog for Fixed {
        fn get(&self, sku: &Sku) -> Result<Option<Product>, Error> {
            Ok(self.0.iter().find(|p| &p.sku == sku).cloned())
        }
        fn put(&self, _: Product) -> Result<(), Error> {
            Err(Error::Store("read-only".into()))
        }
        fn list(&self) -> Result<Vec<Product>, Error> {
            Ok(self.0.clone())
        }
    }

    fn product(sku: &str, cents: u64) -> Product {
        Product {
            sku: sku.parse().unwrap(),
            name: sku.to_lowercase(),
            price: Money::from_cents(cents),
        }
    }

    #[test]
    fn totals_lines() {
        let catalog = Fixed(vec![product("TEA", 450), product("MUG", 1200)]);
        let lines = [("TEA".parse().unwrap(), 2), ("MUG".parse().unwrap(), 1)];
        assert_eq!(order_total(&catalog, &lines), Ok(Money::from_cents(2100)));
    }

    #[test]
    fn unknown_sku_is_not_found() {
        let catalog = Fixed(vec![]);
        let sku: Sku = "TEA".parse().unwrap();
        assert_eq!(order_total(&catalog, &[(sku.clone(), 1)]), Err(Error::NotFound(sku)));
    }

    #[test]
    fn huge_totals_overflow() {
        let catalog = Fixed(vec![product("GEM", u64::MAX)]);
        let lines = [("GEM".parse().unwrap(), 2)];
        assert_eq!(order_total(&catalog, &lines), Err(Error::Overflow));
    }
}
