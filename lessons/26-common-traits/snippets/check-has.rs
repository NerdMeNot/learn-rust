// Backs the table "Which standard types have which trait". Not shown on the page.
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Display};
use std::hash::Hash;

macro_rules! has {
    ($t:ty, $tr:path) => {{
        #[allow(dead_code)]
        trait No {
            const YES: bool = false;
        }
        impl<T: ?Sized> No for T {}
        struct W<T: ?Sized>(std::marker::PhantomData<T>);
        #[allow(dead_code)]
        impl<T: ?Sized + $tr> W<T> {
            const YES: bool = true;
        }
        <W<$t>>::YES
    }};
}

#[allow(dead_code)]
struct NoDefault2;

fn main() {
    // i32
    assert!(has!(i32, Display) && has!(i32, Copy) && has!(i32, Eq) && has!(i32, Ord));
    assert!(has!(i32, Hash) && has!(i32, Default));
    // f64
    assert!(has!(f64, Display) && has!(f64, Copy) && has!(f64, PartialEq) && has!(f64, PartialOrd));
    assert!(!has!(f64, Eq) && !has!(f64, Ord) && !has!(f64, Hash));
    assert!(has!(f64, Default));
    // bool and char
    assert!(has!(bool, Display) && has!(bool, Copy) && has!(bool, Ord) && has!(bool, Hash));
    assert!(has!(char, Display) && has!(char, Copy) && has!(char, Ord) && has!(char, Hash));
    assert!(has!(bool, Default) && has!(char, Default));
    // String
    assert!(has!(String, Display) && has!(String, Eq) && has!(String, Ord) && has!(String, Hash));
    assert!(has!(String, Default) && !has!(String, Copy));
    // &str
    assert!(has!(&str, Display) && has!(&str, Copy) && has!(&str, Ord) && has!(&str, Hash));
    assert!(has!(&str, Default));
    // Vec<T>
    assert!(!has!(Vec<i32>, Display) && !has!(Vec<i32>, Copy));
    assert!(has!(Vec<i32>, Eq) && has!(Vec<i32>, Ord) && has!(Vec<i32>, Hash));
    assert!(has!(Vec<i32>, Default));
    assert!(!has!(Vec<f64>, Eq) && !has!(Vec<f64>, Ord) && !has!(Vec<f64>, Hash));
    assert!(has!(Vec<f64>, PartialOrd));
    // Option<T>
    assert!(!has!(Option<i32>, Display) && has!(Option<i32>, Copy));
    assert!(has!(Option<i32>, Eq) && has!(Option<i32>, Ord) && has!(Option<i32>, Hash));
    assert!(has!(Option<i32>, Default) && !has!(Option<String>, Copy));
    assert!(!has!(Option<f64>, Eq));
    // tuples and arrays
    assert!(!has!((i32, i32), Display) && has!((i32, i32), Copy));
    assert!(has!((i32, i32), Eq) && has!((i32, i32), Ord) && has!((i32, i32), Hash));
    assert!(has!((i32, i32), Default) && !has!((i32, f64), Eq));
    assert!(!has!([i32; 3], Display) && has!([i32; 3], Copy) && has!([i32; 3], Ord));
    assert!(has!([i32; 3], Hash) && has!([i32; 3], Default));
    // maps
    assert!(!has!(HashMap<i32, i32>, Display) && !has!(HashMap<i32, i32>, Copy));
    assert!(has!(HashMap<i32, i32>, Eq) && !has!(HashMap<i32, i32>, Ord));
    assert!(!has!(HashMap<i32, i32>, Hash) && has!(HashMap<i32, i32>, Default));
    assert!(has!(BTreeMap<i32, i32>, Ord) && has!(BTreeMap<i32, i32>, Hash));
    // "if T" cells
    struct NoDefault;
    assert!(has!(Option<String>, Default) && has!(Vec<NoDefault>, Default));
    assert!(has!((i32, i32), Default) && !has!((i32, NoDefault), Default));
    assert!(!has!((i32, String), Copy) && !has!(Vec<i32>, Copy));
    assert!(!has!(HashMap<i32, f64>, Eq));
    assert!(has!(HashMap<String, String>, Eq) && has!(HashMap<NoDefault2, i32>, Default));
    // everything above prints, clones (but Copy ones do not need to be listed) and ==
    fn basics<T: Debug + Clone + PartialEq>() {}
    basics::<i32>();
    basics::<f64>();
    basics::<String>();
    basics::<Vec<f64>>();
    basics::<Option<String>>();
    basics::<(i32, f64)>();
    basics::<HashMap<String, f64>>();
    basics::<&str>();
    println!("all checks passed");
}
