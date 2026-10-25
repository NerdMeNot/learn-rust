// Backs the "what you write today -> in Rust" tables: every Rust form they show.
use std::collections::HashMap;
use std::ptr;

#[derive(Clone, Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct Counter {
    n: u32,
}

impl Counter {
    fn read(&self) -> u32 {
        self.n
    }
    fn bump(&mut self) {
        self.n += 1;
    }
    fn take(self) -> u32 {
        self.n
    }
}

fn change(p: &mut Person) {
    p.age += 1;
}
fn read(p: &Person) -> u32 {
    p.age
}
fn take(p: Person) -> u32 {
    p.age
}
fn bump_all(people: &mut Vec<Person>) {
    for p in people.iter_mut() {
        p.age += 1;
    }
}
fn push_one(items: &mut Vec<i32>) {
    items.push(1);
}

fn main() {
    let mut people = vec![
        Person {
            name: "Ada".into(),
            age: 36,
        },
        Person {
            name: "Linus".into(),
            age: 54,
        },
    ];

    // Go: for _, p := range people  /  people[i].Age++  /  p := &people[0]
    let mut total = 0;
    for p in &people {
        total += p.age;
    }
    assert_eq!(total, 90);
    for p in &mut people {
        p.age += 1;
    }
    let p = &mut people[0];
    p.age += 1;
    assert_eq!(people[0].age, 38);
    assert_eq!(people[1].age, 55);

    // Go: m[k].Age++ is an error; Rust asks for a mutable lookup
    let mut by_name: HashMap<String, Person> = HashMap::new();
    by_name.insert("ada".into(), people[0].clone());
    if let Some(p) = by_name.get_mut("ada") {
        p.age += 1;
    }
    assert_eq!(by_name["ada"].age, 39);

    // Go: for _, p := range people (a copy of each element)
    let mut copies: Vec<Person> = people.iter().cloned().collect();
    assert_eq!(copies, people);
    bump_all(&mut copies);
    assert_eq!(copies[0].age, people[0].age + 1);

    // struct copy: Copy types copy, others need .clone()
    let a = Point { x: 1, y: 2 };
    let mut b = a;
    b.x = 9;
    assert_eq!((a.x, b.x), (1, 9));
    let c = people[0].clone();
    assert_eq!(c, people[0]);

    // parameters: &mut changes the caller's, & reads, plain takes it
    change(&mut people[1]);
    assert_eq!(read(&people[1]), 56);
    assert_eq!(take(people[1].clone()), 56);

    // slices share, clones do not
    let v = vec![1, 2, 3, 4];
    let part = &v[1..3];
    assert!(ptr::eq(part.as_ptr(), &v[1]));
    let owned = v.clone();
    assert!(!ptr::eq(owned.as_ptr(), v.as_ptr()));
    let mut list = Vec::new();
    push_one(&mut list);
    assert_eq!(list, [1]);

    // methods: self, &self, &mut self
    let mut counter = Counter { n: 0 };
    counter.bump();
    assert_eq!(counter.read(), 1);
    assert_eq!(counter.take(), 1);

    // nested clone is deep for owned data
    let grid = vec![vec![1, 2], vec![3, 4]];
    let mut deep = grid.clone();
    deep[0][0] = 99;
    assert_eq!(grid[0][0], 1);

    // C++: for (auto x : v) / for (const auto& x : v) / for (auto& x : v)
    let names = vec![String::from("a"), String::from("b")];
    let copies: Vec<String> = names.iter().cloned().collect();
    assert_eq!(copies, names);
    let mut nums = vec![1, 2];
    for n in nums.iter().copied() {
        assert!(n > 0);
    }
    for n in &mut nums {
        *n += 1;
    }
    assert_eq!(nums, [2, 3]);

    // identity: ptr::eq on references
    let first = &nums[0];
    let same = &nums[0];
    assert!(ptr::eq(first, same));
    let x = 5;
    let y = 5;
    assert!(!ptr::eq(&x, &y));
}
