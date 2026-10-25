use std::cell::Cell;

struct Page {
    title: String,
    views: Cell<u32>,
}

impl Page {
    fn show(&self) -> String {
        self.views.set(self.views.get() + 1); // &self, yet the count changes
        format!("{} (view {})", self.title, self.views.get())
    }
}

fn main() {
    let page = Page {
        title: String::from("Home"),
        views: Cell::new(0),
    };
    println!("{}", page.show());
    println!("{}", page.show());
    println!("replace returns the old value: {}", page.views.replace(10));
    println!("views: {}", page.views.get());
    println!("size_of Cell<u32>: {}", size_of::<Cell<u32>>());
}
