// Backs the table cells on the page: iteration order, duplicates, indexing.
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

fn sorted<T: Ord + Clone>(it: impl IntoIterator<Item = T>) -> Vec<T> {
    let mut v: Vec<T> = it.into_iter().collect();
    v.sort();
    v
}

fn main() {
    // Iteration order
    let v = vec![3, 1, 2];
    assert_eq!(v.iter().copied().collect::<Vec<_>>(), [3, 1, 2]);

    let mut d = VecDeque::new();
    d.push_back(1);
    d.push_back(2);
    d.push_front(0);
    assert_eq!(d.iter().copied().collect::<Vec<_>>(), [0, 1, 2]);

    for order in [[3, 1, 2], [1, 2, 3], [2, 3, 1]] {
        let set: BTreeSet<i32> = order.into_iter().collect();
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), [1, 2, 3]);
        let map: BTreeMap<i32, ()> = order.into_iter().map(|k| (k, ())).collect();
        assert_eq!(map.keys().copied().collect::<Vec<_>>(), [1, 2, 3]);
    }

    let mut heap = BinaryHeap::from([4, 9, 1, 7]);
    assert_eq!(sorted(heap.iter().copied()), [1, 4, 7, 9]); // all there, order unspecified
    assert_eq!(heap.clone().into_sorted_vec(), [1, 4, 7, 9]);
    let mut popped = Vec::new();
    while let Some(n) = heap.pop() {
        popped.push(n);
    }
    assert_eq!(popped, [9, 7, 4, 1]);

    let hs: HashSet<i32> = [4, 9, 1, 7].into_iter().collect();
    assert_eq!(sorted(hs.iter().copied()), [1, 4, 7, 9]); // all there, order unspecified

    // Duplicates
    let mut vv = vec![1];
    vv.push(1);
    assert_eq!(vv.len(), 2);
    let mut dd = VecDeque::from([1]);
    dd.push_back(1);
    assert_eq!(dd.len(), 2);
    let mut h2 = BinaryHeap::from([1]);
    h2.push(1);
    assert_eq!(h2.len(), 2);
    let mut s = HashSet::new();
    assert!(s.insert(1));
    assert!(!s.insert(1));
    let mut bs = BTreeSet::new();
    assert!(bs.insert(1));
    assert!(!bs.insert(1));
    let mut m = HashMap::new();
    assert_eq!(m.insert("k", 1), None);
    assert_eq!(m.insert("k", 2), Some(1));
    assert_eq!(m.len(), 1);
    let mut bm2 = BTreeMap::new();
    assert_eq!(bm2.insert("k", 1), None);
    assert_eq!(bm2.insert("k", 2), Some(1));
    assert_eq!(bm2.len(), 1);
    let mut ll2 = LinkedList::from([1]);
    ll2.push_back(1);
    assert_eq!(ll2.len(), 2);
    // Keys are compared by value: equal tuples are one key
    assert_eq!(HashSet::from([(1, 2), (1, 2)]).len(), 1);

    // BTreeSet has range, first, pop_first and split_off too
    let mut bs2 = BTreeSet::from([10, 20, 30, 40]);
    assert_eq!(bs2.first(), Some(&10));
    assert_eq!(bs2.range(15..35).copied().collect::<Vec<_>>(), [20, 30]);
    assert_eq!(bs2.pop_first(), Some(10));
    assert_eq!(bs2.split_off(&30).len(), 2);

    // Indexing
    assert_eq!(v[0], 3);
    assert_eq!(d[0], 0);
    assert_eq!(m["k"], 2);
    let bm = BTreeMap::from([(1, "a")]);
    assert_eq!(bm[&1], "a");

    // The linked list: both ends, split and append, no position lookup
    let mut l = LinkedList::from([2, 3]);
    l.push_front(1);
    l.push_back(4);
    assert_eq!(l.iter().copied().collect::<Vec<_>>(), [1, 2, 3, 4]);
    assert_eq!(l.pop_front(), Some(1));
    assert_eq!(l.pop_back(), Some(4));
    let mut tail = l.split_off(1);
    assert_eq!((l.len(), tail.len()), (1, 1));
    l.append(&mut tail);
    assert_eq!(l.iter().copied().collect::<Vec<_>>(), [2, 3]);
    assert!(tail.is_empty());
}
