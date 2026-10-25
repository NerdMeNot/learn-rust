mod outer {
    fn outer_secret() -> u32 {
        1
    }

    mod inner {
        fn inner_secret() -> u32 {
            2
        }

        pub fn from_inside() -> u32 {
            // The inner module reads its parent's private function.
            super::outer_secret() + inner_secret()
        }
    }

    pub fn from_outside() -> u32 {
        // The outer module cannot read the inner module's private function.
        inner::inner_secret()
    }
}

fn main() {
    println!("{}", outer::from_outside());
}
