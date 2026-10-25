trait Source {
    async fn fetch(&self) -> u32;
}

fn use_it(_: &dyn Source) {}

fn main() {}
