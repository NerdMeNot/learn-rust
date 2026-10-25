pub mod discounts; // now main.rs can name pricing::discounts

use crate::models::Order; // no `mod models;` here: main.rs declared it, we only name it

pub fn total(order: &Order) -> String {
    let gross = order.unit_cents * order.quantity as u64;
    let net = gross - discounts::bulk(order.quantity, gross);
    format!("${}.{:02}", net / 100, net % 100)
}
