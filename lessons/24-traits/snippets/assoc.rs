use std::any::type_name;

trait Container {
    type Item; // chosen by each implementing type
    const KIND: &'static str; // a constant, also chosen per type

    fn first(&self) -> Option<&Self::Item>;
}

struct Names(Vec<String>);
struct Counts(Vec<u32>);

impl Container for Names {
    type Item = String;
    const KIND: &'static str = "names";

    fn first(&self) -> Option<&String> {
        self.0.first()
    }
}

impl Container for Counts {
    type Item = u32;
    const KIND: &'static str = "counts";

    fn first(&self) -> Option<&u32> {
        self.0.first()
    }
}

fn show<C: Container>(c: &C) {
    println!(
        "{}: Item is {}, first is {}",
        C::KIND,
        type_name::<C::Item>(),
        c.first().is_some()
    );
}

fn main() {
    show(&Names(vec![String::from("ana")]));
    show(&Counts(vec![]));
}
