fn main() {
    let scores = [10, 20, 30];

    // an array can be looped over directly
    for s in scores {
        println!("score: {s}");
    }

    // enumerate pairs each item with its position
    let names = ["ann", "bo"];
    for (i, name) in names.iter().enumerate() {
        println!("name {i}: {name}");
    }
}
