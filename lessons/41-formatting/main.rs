use std::fmt;

struct Money {
    cents: i64,
}

// Display: how Money looks with {}
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}.{:02}", self.cents / 100, self.cents % 100)
    }
}

#[derive(Debug)]
struct Item {
    name: String,
    qty: u32,
}

fn main() {
    let n = 42;
    println!("[{n:>6}] [{n:<6}] [{n:^6}] [{n:06}]");
    println!("{:.2} {:e} {n:#x} {n:#b}", 3.14159, 1234.5);

    let price = Money { cents: 1999 };
    println!("total: {price}");

    let label = format!("{price} each"); // format! builds a String
    println!("{label} ({} chars)", label.len());

    let item = Item {
        name: "pen".to_string(),
        qty: 3,
    };
    println!("{} x {}", item.qty, item.name);
    println!("{item:?}"); // Debug, written by derive
    println!("{item:#?}"); // pretty Debug
}
