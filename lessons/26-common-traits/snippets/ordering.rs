use std::cmp::{Ordering, Reverse};

// Derived: the fields are compared in the order they are written.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct ByRank {
    rank: u8,
    suit: char,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct BySuit {
    suit: char,
    rank: u8,
}

// Derived on an enum: the variants are ordered as they are declared.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Level {
    Low,
    Mid,
    High,
}

// Written by hand: most urgent first, then by name.
#[derive(PartialEq, Eq)]
struct Task {
    urgency: u8,
    name: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Task) -> Ordering {
        other
            .urgency
            .cmp(&self.urgency)
            .then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Task) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let (a, b) = ((1, 'z'), (2, 'a'));
    println!(
        "by rank first: a < b is {}",
        ByRank {
            rank: a.0,
            suit: a.1
        } < ByRank {
            rank: b.0,
            suit: b.1
        }
    );
    println!(
        "by suit first: a < b is {}",
        BySuit {
            suit: a.1,
            rank: a.0
        } < BySuit {
            suit: b.1,
            rank: b.0
        }
    );

    println!("Low < High: {}", Level::Low < Level::High);
    println!("Mid.cmp(Low): {:?}", Level::Mid.cmp(&Level::Low));

    let mut tasks = vec![
        Task {
            urgency: 1,
            name: String::from("sweep"),
        },
        Task {
            urgency: 9,
            name: String::from("deploy"),
        },
        Task {
            urgency: 9,
            name: String::from("alert"),
        },
    ];
    tasks.sort();
    for t in &tasks {
        println!("{} {}", t.urgency, t.name);
    }

    let mut nums = vec![3, 1, 2];
    nums.sort_by_key(|&n| Reverse(n));
    println!("reverse: {nums:?}");
}
