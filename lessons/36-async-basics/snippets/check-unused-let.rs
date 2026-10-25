// The lint fires on a discarded statement only: neither of these is reported.
#![deny(unused_must_use)]

async fn save() {
    println!("saving");
}

fn main() {
    let _ = save();
    drop(save());
    let _kept = save();
}
