// No `mod` lines here. main.rs already declared this file, once.

pub struct Order {
    pub customer: String,
    pub quantity: u32,
    pub unit_cents: u64,
}

impl Order {
    pub fn new(customer: &str, quantity: u32, unit_cents: u64) -> Order {
        Order {
            customer: customer.to_string(),
            quantity,
            unit_cents,
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "{} x {} cents for {}",
            self.quantity, self.unit_cents, self.customer
        )
    }
}
