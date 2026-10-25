// Backs the "in Rust" cells of the tables in lesson.mdx. Never shown.
fn main() {
    let mut v = vec![3, 1, 2];

    v.push(4);
    assert_eq!(v, [3, 1, 2, 4]);
    assert_eq!(v.pop(), Some(4));
    assert_eq!(Vec::<i32>::new().pop(), None);

    v.extend(vec![7, 8]);
    v.extend_from_slice(&[9]);
    assert_eq!(v, [3, 1, 2, 7, 8, 9]);

    assert_eq!(vec![0; 3], [0, 0, 0]);
    assert_eq!(Vec::<i32>::with_capacity(5).capacity(), 5);
    assert_eq!(v.last(), Some(&9));
    assert_eq!(v.first(), Some(&3));
    assert_eq!(v.get(99), None);

    v.insert(1, 5);
    assert_eq!(v, [3, 5, 1, 2, 7, 8, 9]);
    assert_eq!(v.remove(1), 5);
    assert_eq!(&v[1..3], [1, 2]);
    assert_eq!(v[1..3].to_vec(), vec![1, 2]);

    assert_eq!(v.contains(&7), true);
    assert_eq!(v.iter().position(|&e| e == 7), Some(3));
    assert_eq!(v.iter().position(|&e| e == 99), None);

    let mut sorted_copy = v.clone();
    sorted_copy.sort();
    assert_eq!(sorted_copy, [1, 2, 3, 7, 8, 9]);
    assert_eq!(v, [3, 1, 2, 7, 8, 9]); // the original is unchanged
    assert_eq!(sorted_copy.binary_search(&7), Ok(3));
    assert_eq!(sorted_copy.binary_search(&5), Err(3));

    sorted_copy.sort_by(|a, b| b.cmp(a));
    assert_eq!(sorted_copy, [9, 8, 7, 3, 2, 1]);
    sorted_copy.reverse();
    assert_eq!(sorted_copy, [1, 2, 3, 7, 8, 9]);

    let mut words = vec!["kiwi", "fig", "banana"];
    words.sort_by_key(|w| w.len());
    assert_eq!(words, ["fig", "kiwi", "banana"]);

    v.retain(|x| *x != 7);
    assert_eq!(v, [3, 1, 2, 8, 9]);
    v.truncate(2);
    assert_eq!(v, [3, 1]);

    let mut runs = vec![1, 1, 2, 2, 1];
    runs.dedup();
    assert_eq!(runs, [1, 2, 1]);

    assert_eq!([vec![1], vec![2, 3]].concat(), [1, 2, 3]);
    assert_eq!(["a", "b"].join("-"), "a-b");

    let (left, right) = [1, 2, 3, 4].split_at(1);
    assert_eq!((left, right), (&[1][..], &[2, 3, 4][..]));

    let mut seen = 0;
    for (i, x) in v.iter().enumerate() {
        seen += i as i32 + x;
    }
    assert_eq!(seen, 5);

    let copy = v.clone();
    assert_eq!(copy, v);
    // Rows added in the second pass.
    let mut x = vec![1, 2, 3, 4, 5];
    let drained: Vec<i32> = x.drain(1..3).collect();
    assert_eq!((drained, x.clone()), (vec![2, 3], vec![1, 4, 5]));
    x.resize(5, 0);
    assert_eq!(x, [1, 4, 5, 0, 0]);
    x.resize(2, 0);
    assert_eq!(x, [1, 4]);
    let w = vec![8, 9];
    x.extend(&w);
    x.extend_from_slice(&w);
    assert_eq!((x.len(), w.len()), (6, 2));
    let (a, b) = (vec![1], vec![2]);
    assert_eq!([a.as_slice(), b.as_slice()].concat(), [1, 2]);
    assert_eq!((a.len(), b.len()), (1, 1));
    let mut dst = [0; 3];
    dst.copy_from_slice(&[7, 8, 9]);
    assert_eq!(dst, [7, 8, 9]);
    assert_eq!(x.iter().sum::<i32>(), 1 + 4 + 8 + 9 + 8 + 9);
    assert_eq!(x.iter().max(), Some(&9));
    let mut r = vec![1, 2, 3];
    r.reverse();
    assert_eq!(r, [3, 2, 1]);
    let mut reserved = vec![1, 2, 3];
    reserved.reserve(5);
    assert!(reserved.capacity() >= 8);

    // Calls the lesson says panic. (The panic messages go to stderr.)
    use std::panic::catch_unwind;
    assert!(catch_unwind(|| vec![1, 2, 3][1..9].len()).is_err());
    assert!(catch_unwind(|| vec![1, 2, 3].windows(0).count()).is_err());
    assert!(catch_unwind(|| vec![1, 2, 3].chunks(0).count()).is_err());
    assert!(catch_unwind(|| vec![1, 2, 3].insert(4, 0)).is_err());
    assert!(catch_unwind(|| vec![1, 2, 3].remove(30)).is_err());
    assert!(
        catch_unwind(|| {
            let v: Vec<i32> = Vec::new();
            v[v.len() - 1]
        })
        .is_err()
    );
    assert!(
        catch_unwind(|| {
            let mut d = [0; 2];
            d.copy_from_slice(&[1, 2, 3]);
        })
        .is_err()
    );
    assert!(
        catch_unwind(|| {
            let mut v = vec![1, 2, 3];
            v[6] = 70;
        })
        .is_err()
    );
    println!("all checks passed");
}
