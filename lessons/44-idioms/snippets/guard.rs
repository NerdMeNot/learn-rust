use std::cell::Cell;

/// Raises a counter while it is alive. The matching decrement is in `drop`, so it
/// cannot be forgotten and runs on every way out of the scope.
struct Depth<'a> {
    level: &'a Cell<u32>,
}

impl<'a> Depth<'a> {
    fn enter(level: &'a Cell<u32>, name: &str) -> Depth<'a> {
        level.set(level.get() + 1);
        println!("{}enter {name}", "  ".repeat(level.get() as usize - 1));
        Depth { level }
    }
}

impl Drop for Depth<'_> {
    fn drop(&mut self) {
        self.level.set(self.level.get() - 1);
        println!("{}leave", "  ".repeat(self.level.get() as usize));
    }
}

fn parse(level: &Cell<u32>, text: &str) -> Result<u32, std::num::ParseIntError> {
    let _guard = Depth::enter(level, "parse");
    let n = text.parse::<u32>()?; // an early return: the guard still drops
    Ok(n * 2)
}

fn main() {
    let level = Cell::new(0);
    {
        let _outer = Depth::enter(&level, "main");
        println!("{:?}", parse(&level, "21"));
        println!("{:?}", parse(&level, "x"));
    }
    println!("level after: {}", level.get());
}
