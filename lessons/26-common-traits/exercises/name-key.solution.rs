use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

// A name that ignores ASCII case: "Ana", "ANA" and "ana" are one key.
#[derive(Debug, Clone)]
struct Name(String);

impl PartialEq for Name {
    fn eq(&self, other: &Name) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Eq for Name {}

impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for b in self.0.bytes() {
            b.to_ascii_lowercase().hash(state);
        }
    }
}

// All 2^8 spellings of "abcdefgh" with some letters in capitals.
fn spellings() -> Vec<Name> {
    (0..256)
        .map(|mask: u32| {
            let text: String = "abcdefgh"
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if mask >> i & 1 == 1 {
                        c.to_ascii_uppercase()
                    } else {
                        c
                    }
                })
                .collect();
            Name(text)
        })
        .collect()
}

fn main() {
    assert_eq!(Name(String::from("Ana")), Name(String::from("aNA")));
    assert_ne!(Name(String::from("Ana")), Name(String::from("Anna")));
    assert_ne!(Name(String::from("Anab")), Name(String::from("Ana"))); // a prefix is not equal
    assert_ne!(Name(String::from("Ana")), Name(String::from("Bob"))); // same length, other name

    let mut seen = HashSet::new();
    for name in spellings() {
        seen.insert(name);
    }
    assert_eq!(seen.len(), 1); // one name, however it is spelled
    seen.insert(Name(String::from("abcdefgi")));
    assert_eq!(seen.len(), 2);

    let mut ages = HashMap::new();
    ages.insert(Name(String::from("Ana")), 30);
    ages.insert(Name(String::from("Bo")), 25);
    assert_eq!(ages.get(&Name(String::from("ANA"))), Some(&30));
    assert_eq!(ages.get(&Name(String::from("bo"))), Some(&25));
    assert_eq!(ages.get(&Name(String::from("cy"))), None);
    assert_eq!(ages.len(), 2);
    println!("all checks passed");
}
