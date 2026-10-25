mod text_ext {
    pub trait StrExt {
        fn is_blank(&self) -> bool;
    }

    impl StrExt for str {
        fn is_blank(&self) -> bool {
            self.trim().is_empty()
        }
    }
}

fn main() {
    println!("{}", "   ".is_blank());
}
