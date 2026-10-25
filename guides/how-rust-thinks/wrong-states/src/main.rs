// A payment is exactly one of these. Each kind carries only its own data.
enum Payment {
    Cash,
    Card { last4: String },
    Voucher(u32),
}

fn describe(p: &Payment) -> String {
    // match must cover every kind. Add a new kind later and this stops compiling
    // until you handle it.
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
    ];
    for p in &payments {
        println!("{}", describe(p));
    }
}
