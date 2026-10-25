use unicode_segmentation::GraphemeCursor;

fn main() {
    let word = "héllo👋🏽";
    // the crate is mentioned here, but the trait that adds `.graphemes()` is not imported
    let _cursor = GraphemeCursor::new(0, word.len(), true);
    println!("graphemes: {}", word.graphemes(true).count());
}
