#![deny(private_interfaces)]

mod shop {
    struct Cents(u32); // private

    // A public function whose signature names a private type.
    pub fn price() -> Cents {
        Cents(250)
    }
}

fn main() {
    println!("started");
}
