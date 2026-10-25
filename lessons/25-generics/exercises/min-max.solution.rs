// The smallest and the largest item of a slice, or None if it is empty.
// The checks use whole numbers, decimals, &str, char and a type with almost no traits.
fn min_max<T: PartialOrd>(items: &[T]) -> Option<(&T, &T)> {
    let mut smallest = items.first()?;
    let mut largest = smallest;
    for item in items {
        if item < smallest {
            smallest = item;
        }
        if item > largest {
            largest = item;
        }
    }
    Some((smallest, largest))
}

// Comparable with < and >, and nothing else: no Copy, Clone, Debug or Ord.
#[derive(PartialEq, PartialOrd)]
struct Version(u32, u32);

fn main() {
    assert_eq!(min_max(&[3, 9, 2, 7]), Some((&2, &9)));
    assert_eq!(min_max(&[1, 5, 9]), Some((&1, &9)));
    assert_eq!(min_max(&[9, 5, 1]), Some((&1, &9)));
    assert_eq!(min_max(&[4]), Some((&4, &4)));
    // Decimals can be compared with < but are not Ord; all negative on purpose.
    assert_eq!(min_max(&[-4.5, -0.5, -9.25]), Some((&-9.25, &-0.5)));
    assert_eq!(
        min_max(&["kiwi", "apple", "pear"]),
        Some((&"apple", &"pear"))
    );
    assert_eq!(min_max(&['z', 'a']), Some((&'a', &'z')));
    assert_eq!(min_max::<i32>(&[]), None);

    let versions = [Version(1, 4), Version(0, 9), Version(2, 0), Version(1, 10)];
    let (lowest, highest) = min_max(&versions).unwrap();
    assert_eq!((lowest.0, lowest.1), (0, 9));
    assert_eq!((highest.0, highest.1), (2, 0));
    // The answer is an item of the slice itself, not a copy.
    assert!(std::ptr::eq(lowest, &versions[1]));
    println!("all checks passed");
}
