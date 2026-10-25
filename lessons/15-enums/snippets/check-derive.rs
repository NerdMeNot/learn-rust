use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
enum Priority {
    Low,
    #[default]
    Normal,
    Urgent,
}

fn main() {
    // Hash + Eq: usable in a set
    let set: HashSet<Priority> = [Priority::Low, Priority::Low, Priority::Urgent]
        .into_iter()
        .collect();
    assert_eq!(set.len(), 2);
    // Ord: sorting goes by declaration order
    let mut v = vec![Priority::Urgent, Priority::Low, Priority::Normal];
    v.sort();
    assert_eq!(v, [Priority::Low, Priority::Normal, Priority::Urgent]);
    // Default: the variant marked #[default]
    assert_eq!(Priority::default(), Priority::Normal);
    // Copy: using q does not move p
    let p = Priority::Low;
    let q = p;
    assert_eq!(p, q);
    // explicit numbers decide the order, not the order of writing
    #[derive(PartialEq, PartialOrd)]
    enum Rank {
        Top = 2,
        Bottom = 1,
    }
    assert!(Rank::Bottom < Rank::Top);
    // PartialOrd with data: the variant comes first, then the fields
    #[derive(PartialEq, PartialOrd)]
    enum Reply {
        Ok,
        Number(f64),
    }
    assert!(Reply::Ok < Reply::Number(-1.0));
    assert!(Reply::Number(1.0) < Reply::Number(2.0));
}
