mod metric {
    pub fn describe() -> &'static str {
        "metres"
    }
}

mod imperial {
    pub fn describe() -> &'static str {
        "feet"
    }
}

use imperial::*;
use metric::*;

fn main() {
    println!("describe: {}", describe());
}
