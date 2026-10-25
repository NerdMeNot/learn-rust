// Backs the loop table: what each form gives you, and what happens to the Vec.
use std::any::type_name_of_val;

#[derive(Clone, Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // copies: changing x leaves the Vec alone
    let v = vec![1, 2, 3];
    let mut bumped = Vec::new();
    for mut x in v.iter().copied() {
        x += 1;
        bumped.push(x);
    }
    for mut x in v.clone() {
        x += 1;
        bumped.push(x);
    }
    assert_eq!(bumped, [2, 3, 4, 2, 3, 4]);
    assert_eq!(v, [1, 2, 3]);

    // &v and iter(): shared references, reading only
    let mut sum = 0;
    for x in &v {
        sum += x;
    }
    for x in v.iter() {
        sum += *x;
    }
    assert_eq!(sum, 12);

    // &mut v and iter_mut(): the elements themselves
    let mut w = vec![1, 2, 3];
    for x in &mut w {
        *x += 1;
    }
    for x in w.iter_mut() {
        *x += 10;
    }
    assert_eq!(w, [12, 13, 14]);

    // into_iter on a Vec: the items come out by value, and the Vec is consumed
    let items = vec![String::from("a"), String::from("b")];
    let mut seen = Vec::new();
    for s in items {
        assert_eq!(type_name_of_val(&s), "alloc::string::String");
        seen.push(s);
    }
    assert_eq!(seen.len(), 2);

    // an array of Copy items is itself Copy: the loop uses a copy and the array stays
    let arr = [1, 2, 3];
    let mut from_array = Vec::new();
    for mut x in arr {
        x += 1;
        from_array.push(x);
    }
    assert_eq!(from_array, [2, 3, 4]);
    assert_eq!(arr, [1, 2, 3]);

    // struct elements: x.age through & and &mut
    let mut people = vec![Person {
        name: "Ada".into(),
        age: 36,
    }];
    for p in &people {
        assert_eq!(p.age, 36);
    }
    for p in &mut people {
        p.age += 1;
    }
    assert_eq!(people[0].age, 37);
}
