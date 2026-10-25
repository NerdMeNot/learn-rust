#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

// `impl<T>` declares T, then `Pair<T>` uses it: every Pair gets these methods.
impl<T> Pair<T> {
    fn new(first: T, second: T) -> Pair<T> {
        Pair { first, second }
    }

    fn swapped(self) -> Pair<T> {
        Pair::new(self.second, self.first)
    }

    // A method can add parameters of its own, after its name.
    fn map<U>(self, f: impl Fn(T) -> U) -> Pair<U> {
        Pair::new(f(self.first), f(self.second))
    }
}

// No T here: this method exists only on Pair<f64>.
impl Pair<f64> {
    fn sum(&self) -> f64 {
        self.first + self.second
    }
}

// Two parameters, which may be different types.
struct Entry<K, V> {
    key: K,
    value: V,
}

#[derive(Debug)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

fn main() {
    println!("ints: {:?}", Pair::new(1, 2).swapped());
    println!("mapped: {:?}", Pair::new(1, 2).map(|n| format!("#{n}")));
    println!("words: {:?}", Pair::new("a", "b"));
    println!("sum: {}", Pair::new(1.5, 2.0).sum());
    let entry = Entry {
        key: "id",
        value: 7,
    };
    println!("entry: {} -> {}", entry.key, entry.value);
    let found: Maybe<char> = Maybe::Just('x');
    let missing: Maybe<char> = Maybe::Nothing;
    println!("maybe: {found:?} and {missing:?}");
}
