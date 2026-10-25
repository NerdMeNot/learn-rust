/// The smallest rectangle that holds every point, as (bottom_left, top_right).
/// `points` is never empty.
fn bounding_box(points: &[(i32, i32)]) -> ((i32, i32), (i32, i32)) {
    todo!("start both corners at points[0]; `for &(x, y) in points` unpacks each point")
}

fn main() {
    assert_eq!(bounding_box(&[(1, 2)]), ((1, 2), (1, 2)));
    assert_eq!(bounding_box(&[(4, 4), (4, 4)]), ((4, 4), (4, 4)));
    // Points on a line.
    assert_eq!(bounding_box(&[(0, 5), (3, 5)]), ((0, 5), (3, 5)));
    // The smallest x and the smallest y come from different points.
    assert_eq!(
        bounding_box(&[(3, -1), (-2, 5), (0, 0)]),
        ((-2, -1), (3, 5))
    );

    // The caller unpacks a tuple of tuples in one step.
    let ((x0, y0), (x1, y1)) = bounding_box(&[(0, 0), (6, 4)]);
    assert_eq!((x1 - x0) * (y1 - y0), 24);

    println!("all checks passed");
}
