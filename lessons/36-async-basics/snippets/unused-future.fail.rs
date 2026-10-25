// By default this is a warning. `deny` turns it into the error the page can show.
#![deny(unused_must_use)]

async fn save() {
    println!("saving");
}

fn main() {
    save();
    println!("done");
}
