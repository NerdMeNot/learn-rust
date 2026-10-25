use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // A struct key: derive Hash, PartialEq and Eq. Keys are compared by value.
    let mut names: HashMap<Point, &str> = HashMap::new();
    names.insert(Point { x: 0, y: 0 }, "origin");
    println!("Point 0,0: {:?}", names.get(&Point { x: 0, y: 0 }));
    println!("Point 1,0: {:?}", names.get(&Point { x: 1, y: 0 }));

    // A tuple is a key if all its parts are.
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    grid.insert((1, 2), '#');
    println!("(1, 2): {:?}", grid.get(&(1, 2)));

    // So is a Vec, if its items are.
    let mut visits: HashMap<Vec<&str>, u32> = HashMap::new();
    visits.insert(vec!["usr", "bin"], 3);
    println!("[usr, bin]: {:?}", visits.get(&vec!["usr", "bin"]));

    // Option works as a key, so "no key" can be one of the keys.
    let mut by_owner: HashMap<Option<&str>, u32> = HashMap::new();
    by_owner.insert(None, 1);
    by_owner.insert(Some("ana"), 2);
    println!("None: {:?}", by_owner.get(&None));
}
