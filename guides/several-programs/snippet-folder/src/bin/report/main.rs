// A program that is a folder: its own modules live next to this file.
mod table;

fn main() {
    println!("{}", tools::banner("report"));
    table::print(&[("seed", 3), ("report", 1)]);
}
