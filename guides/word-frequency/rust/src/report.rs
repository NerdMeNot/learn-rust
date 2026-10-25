/// Prints the totals, then one line per row: rank, word, count.
pub fn print(total: usize, distinct: usize, rows: &[(String, usize)]) {
    println!("{total} words, {distinct} distinct");
    let width = rows
        .iter()
        .map(|(word, _)| word.chars().count())
        .max()
        .unwrap_or(0);
    for (i, (word, count)) in rows.iter().enumerate() {
        println!("{:>3}  {word:<width$}  {count:>5}", i + 1);
    }
}
