use anyhow::{Context, Result};
use std::{env, fs};

fn main() -> Result<()> {
    let path = env::args().nth(1).unwrap_or_else(|| "orders.csv".to_string());
    let text = fs::read_to_string(&path).with_context(|| format!("reading {path}"))?;
    let orders = tally::parse(&text).with_context(|| format!("parsing {path}"))?;
    let cents = tally::total_cents(&orders);
    println!("{} orders, total {}.{:02}", orders.len(), cents / 100, cents % 100);
    Ok(())
}
