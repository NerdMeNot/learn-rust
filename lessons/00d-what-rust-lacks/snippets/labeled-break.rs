fn main() {
    let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut found = None;
    'search: for (row, cells) in grid.iter().enumerate() {
        for (col, &cell) in cells.iter().enumerate() {
            if cell == 5 {
                found = Some((row, col));
                break 'search;
            }
        }
    }
    println!("found: {found:?}");
}
