mod bank {
    fn secret_rate() -> u32 {
        5
    }

    pub fn rate() -> u32 {
        secret_rate()
    }
}

fn main() {
    println!("rate: {}", bank::rate());
    println!("secret: {}", bank::secret_rate());
}
