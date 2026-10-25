fn main() {
    let mut grid = vec![vec![0; 3]; 2];
    grid[1][2] = 7;
    println!("grid: {grid:?}");
    println!("rows: {}, columns: {}", grid.len(), grid[0].len());
    println!(
        "each row has its own buffer: {}",
        grid[0].as_ptr() != grid[1].as_ptr()
    );

    // One flat Vec and some arithmetic: a single buffer.
    let (rows, cols) = (2, 3);
    let (row, col) = (1, 2);
    let mut flat = vec![0; rows * cols];
    flat[row * cols + col] = 7;
    println!("flat: {flat:?}");

    // Rows may have different lengths.
    let ragged = vec![vec![1], vec![2, 3], vec![]];
    for (i, r) in ragged.iter().enumerate() {
        println!("row {i} has {} items", r.len());
    }
}
