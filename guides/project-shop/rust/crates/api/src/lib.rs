//! The shop's API: one text request in, a status and a body out. There is no
//! network here; a server would call [`Api::handle`] for each request it reads.
//!
//! | Request | Meaning |
//! | --- | --- |
//! | `PUT <sku> <cents> <name...>` | add or replace a product |
//! | `GET <sku>` | show a product |
//! | `LIST` | show every product |
//! | `TOTAL <sku>:<quantity> ...` | price an order |

use shop_core::{Catalog, Error, Money, Product, Sku, order_total};

/// What [`Api::handle`] returns. The status numbers follow HTTP's meanings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    /// 200 ok, 201 created, 400 bad request, 404 not found, 500 server error.
    pub status: u16,
    /// Text for the caller.
    pub body: String,
}

/// The handler. It owns a catalog behind the [`Catalog`] trait and does not
/// know which kind.
pub struct Api {
    catalog: Box<dyn Catalog>,
}

impl Api {
    /// An API on top of `catalog`.
    pub fn new(catalog: Box<dyn Catalog>) -> Api {
        Api { catalog }
    }

    /// Runs one request line.
    pub fn handle(&self, request: &str) -> Response {
        match self.run(request) {
            Ok(response) => response,
            Err(e) => Response { status: status_of(&e), body: e.to_string() },
        }
    }

    fn run(&self, request: &str) -> Result<Response, Error> {
        // Up to four pieces: the name of a PUT is everything after the price.
        let parts: Vec<&str> = request.trim().splitn(4, ' ').collect();
        match parts.as_slice() {
            ["PUT", sku, cents, name] => {
                let sku: Sku = sku.parse()?;
                let price: Money = cents.parse()?;
                self.catalog.put(Product { sku, name: name.trim().to_string(), price })?;
                Ok(Response { status: 201, body: "stored".into() })
            }
            ["GET", sku] => {
                let sku: Sku = sku.parse()?;
                let product = self.catalog.get(&sku)?.ok_or(Error::NotFound(sku))?;
                Ok(Response { status: 200, body: product.to_string() })
            }
            ["LIST"] => {
                let lines: Vec<String> =
                    self.catalog.list()?.iter().map(Product::to_string).collect();
                Ok(Response { status: 200, body: lines.join("\n") })
            }
            ["TOTAL", _, ..] => {
                let mut lines = Vec::new();
                for word in request.split_whitespace().skip(1) {
                    let (sku, quantity) = word.split_once(':').unwrap_or((word, "1"));
                    let quantity: u32 = quantity
                        .parse()
                        .ok()
                        .filter(|q| *q > 0)
                        .ok_or_else(|| Error::InvalidQuantity(quantity.to_string()))?;
                    lines.push((sku.parse()?, quantity));
                }
                let total = order_total(&*self.catalog, &lines)?;
                Ok(Response { status: 200, body: total.to_string() })
            }
            _ => Ok(Response { status: 400, body: format!("unknown request {request:?}") }),
        }
    }
}

fn status_of(error: &Error) -> u16 {
    match error {
        Error::NotFound(_) => 404,
        Error::Store(_) => 500,
        _ => 400,
    }
}

#[cfg(test)]
mod tests {
    use shop_storage::MemoryCatalog;

    use super::*;

    fn api() -> Api {
        Api::new(Box::new(MemoryCatalog::new()))
    }

    fn reply(api: &Api, request: &str) -> (u16, String) {
        let r = api.handle(request);
        (r.status, r.body)
    }

    #[test]
    fn put_then_get() {
        let api = api();
        assert_eq!(reply(&api, "PUT TEA 450 Green tea, 100 g"), (201, "stored".into()));
        assert_eq!(reply(&api, "GET TEA"), (200, "TEA 4.50 Green tea, 100 g".into()));
    }

    #[test]
    fn list_is_in_sku_order() {
        let api = api();
        api.handle("PUT TEA 450 Tea");
        api.handle("PUT MUG 1200 Mug");
        assert_eq!(reply(&api, "LIST").1, "MUG 12.00 Mug\nTEA 4.50 Tea");
    }

    #[test]
    fn total_prices_an_order() {
        let api = api();
        api.handle("PUT TEA 450 Tea");
        api.handle("PUT MUG 1200 Mug");
        assert_eq!(reply(&api, "TOTAL TEA:2 MUG"), (200, "21.00".into()));
    }

    #[test]
    fn errors_become_statuses() {
        let api = api();
        assert_eq!(reply(&api, "GET TEA").0, 404);
        assert_eq!(reply(&api, "GET tea").0, 400);
        assert_eq!(reply(&api, "PUT TEA 4.50 Tea").0, 400);
        assert_eq!(reply(&api, "TOTAL TEA:0").0, 400);
        assert_eq!(reply(&api, "DELETE TEA").0, 400);
    }
}
