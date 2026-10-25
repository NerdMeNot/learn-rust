mod text_ext {
    /// Our own trait, so we may implement it for the foreign type `str`.
    pub trait StrExt {
        fn is_blank(&self) -> bool;
        fn first_word(&self) -> &str;
    }

    impl StrExt for str {
        fn is_blank(&self) -> bool {
            self.trim().is_empty()
        }

        fn first_word(&self) -> &str {
            self.split_whitespace().next().unwrap_or("")
        }
    }
}

// The methods exist only where the trait is in scope.
use text_ext::StrExt;

fn main() {
    println!("{}", "   ".is_blank());
    println!("{}", "  hello world".first_word());
    let owned = String::from("rust language");
    println!("{}", owned.first_word()); // a String derefs to str
}
