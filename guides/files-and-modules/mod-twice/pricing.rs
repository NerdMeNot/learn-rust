mod discounts;
mod models; // a second, different module: pricing::models
use crate::models::Order;

pub fn total(order: &Order) -> String {
    let gross = order.unit_cents * order.quantity as u64;
    let net = gross - discounts::bulk(order.quantity, gross);
    format!("${}.{:02}", net / 100, net % 100)
}
