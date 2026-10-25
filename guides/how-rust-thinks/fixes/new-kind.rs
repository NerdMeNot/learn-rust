// The same program as wrong-states, with one new kind of payment added.
enum Payment {
    Cash,
    Card { last4: String },
    Voucher(u32),
    Crypto(String),
}

fn describe(p: &Payment) -> String {
    match p {
        Payment::Cash => "cash".to_string(),
        Payment::Card { last4 } => format!("card ending {last4}"),
        Payment::Voucher(code) => format!("voucher #{code}"),
    }
}

fn main() {
    let payments = [
        Payment::Cash,
        Payment::Card {
            last4: "4242".to_string(),
        },
        Payment::Voucher(7),
        Payment::Crypto("bc1q".to_string()),
    ];
    for p in &payments {
        println!("{}", describe(p));
    }
}
