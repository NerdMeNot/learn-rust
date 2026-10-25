// Backs the table cells in lesson.mdx. Never shown.
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn through_mut_ref(v: &mut Vec<i32>) {
    for x in v {
        assert_eq!(type_of(&x), "&mut i32");
    }
}

fn over_slice(s: &[i32]) {
    for x in s {
        assert_eq!(type_of(&x), "&i32");
    }
}

fn drain_all(v: &mut Vec<String>) -> usize {
    let mut n = 0;
    for s in v.drain(..) {
        n += s.len();
    }
    n
}

struct Person {
    age: u32,
}

fn main() {
    let mut v = vec![1, 2, 3];
    through_mut_ref(&mut v);

    // an array of Copy items is copied into the loop and stays usable; a slice is a reference
    let arr = [1, 2, 3];
    for x in arr {
        assert_eq!(type_of(&x), "i32");
    }
    assert_eq!(arr.len(), 3);
    over_slice(&arr);
    over_slice(&v);

    // cloned() works for Strings; copied() is the same for Copy items
    let words = vec![String::from("a"), String::from("bc")];
    let c: Vec<String> = words.iter().cloned().collect();
    assert_eq!(c, words);
    let n: Vec<i32> = v.iter().copied().collect();
    assert_eq!(n, v);

    // drain(..) and mem::take both give owned items from a &mut Vec; drain + break empties it
    let mut w = words.clone();
    assert_eq!(drain_all(&mut w), 3);
    assert!(w.is_empty());
    let mut w = words.clone();
    let mut total = 0;
    for s in std::mem::take(&mut w) {
        total += s.len();
    }
    assert_eq!(total, 3);
    assert!(w.is_empty());
    let mut w = words.clone();
    for _s in w.drain(..) {
        break;
    }
    assert!(w.is_empty());

    // &mut list lets you change each object; HashMap::retain
    let mut people = vec![Person { age: 1 }];
    for p in &mut people {
        p.age += 1;
    }
    assert_eq!(people[0].age, 2);
    let mut ages = std::collections::HashMap::from([("a", 1), ("b", 2)]);
    ages.retain(|_, a| *a > 1);
    assert_eq!(ages.len(), 1);

    // the three forms and what x is
    for x in &v {
        assert_eq!(type_of(&x), "&i32");
    }
    for x in &mut v {
        assert_eq!(type_of(&x), "&mut i32");
    }
    for x in v.clone() {
        assert_eq!(type_of(&x), "i32");
    }
    // arrays and ranges loop by value
    for x in [1, 2] {
        assert_eq!(type_of(&x), "i32");
    }
    for x in 0..2 {
        assert_eq!(type_of(&x), "i32");
    }

    // cloned() copies each item out of the list
    let copies: Vec<i32> = v.iter().cloned().collect();
    assert_eq!(copies, v);
    // &v is v.iter(), &mut v is v.iter_mut(), v is v.into_iter()
    let a: Vec<&i32> = (&v).into_iter().collect();
    let b: Vec<&i32> = v.iter().collect();
    assert_eq!(a, b);
    for x in v.iter_mut() {
        *x += 1;
    }
    assert_eq!(v, [2, 3, 4]);

    // enumerate, zip (stops at the shorter), rev, step_by, skip
    let names = ["ana", "bo", "cy"];
    let ages = [31, 27];
    let e: Vec<(usize, &&str)> = names.iter().enumerate().collect();
    assert_eq!(e, [(0, &"ana"), (1, &"bo"), (2, &"cy")]);
    let z: Vec<(&&str, &i32)> = names.iter().zip(&ages).collect();
    assert_eq!(z.len(), 2);
    let r: Vec<&&str> = names.iter().rev().collect();
    assert_eq!(r, [&"cy", &"bo", &"ana"]);
    let s: Vec<&&str> = names.iter().step_by(2).collect();
    assert_eq!(s, [&"ana", &"cy"]);
    let k: Vec<&&str> = names.iter().skip(1).collect();
    assert_eq!(k, [&"bo", &"cy"]);
    let n = [1, 2, 3, 4, 5];
    assert_eq!(n.windows(2).count(), 4);
    assert_eq!(n.chunks(2).count(), 3);
    assert_eq!(n.chunks(2).last(), Some(&[5][..]));
    assert_eq!(n.chunks_exact(2).count(), 2);
    // windows and chunks on a list shorter than the size
    assert_eq!([1].windows(2).count(), 0);
    assert_eq!(Vec::<i32>::new().chunks(2).count(), 0);
    // iter_mut + enumerate
    let mut m = [0, 0, 0];
    for (i, x) in m.iter_mut().enumerate() {
        *x = i * 10;
    }
    assert_eq!(m, [0, 10, 20]);
    // the one-by-one loop and the index loop visit the same items
    let mut by_index = vec![];
    for i in 0..n.len() {
        by_index.push(n[i]);
    }
    let by_ref: Vec<i32> = n.iter().copied().collect();
    assert_eq!(by_index, by_ref);
    // an empty list: 0..len is empty, but len - 1 underflows
    assert_eq!((0..Vec::<i32>::new().len()).count(), 0);
    assert_eq!(Vec::<i32>::new().len().checked_sub(1), None);
    // a slice of a list loops too
    let part: Vec<&i32> = n[1..3].iter().collect();
    assert_eq!(part, [&2, &3]);

    // retain, and (0..len).rev() removal
    let mut w = vec![1, 2, 3, 4];
    w.retain(|x| x % 2 == 1);
    assert_eq!(w, [1, 3]);
    // for (i, c) in char_indices: byte offsets
    let offsets: Vec<usize> = "héllo".char_indices().map(|(i, _)| i).collect();
    assert_eq!(offsets, [0, 1, 3, 4, 5]);
    assert_eq!("héllo".chars().count(), 5);
    assert_eq!("héllo".len(), 6);
    // a HashMap: pairs of references, and &mut gives a writable value
    let mut m = std::collections::HashMap::new();
    m.insert("a", 1);
    for (k, c) in &m {
        assert_eq!(type_of(&(k, c)), "(&&str, &i32)");
    }
    for (_, c) in &mut m {
        *c += 1;
    }
    assert_eq!(m["a"], 2);
    println!("ok");
}
