// One closure type per struct: the type parameter `F` is that closure's type.
struct Button<F: Fn(&str)> {
    label: String,
    on_click: F,
}

impl<F: Fn(&str)> Button<F> {
    fn click(&self) {
        (self.on_click)(&self.label);
    }
}

// Many closures of different types: box each one.
struct Events {
    handlers: Vec<Box<dyn FnMut(&str) -> usize>>,
}

impl Events {
    fn fire(&mut self, event: &str) -> Vec<usize> {
        self.handlers.iter_mut().map(|h| h(event)).collect()
    }
}

fn main() {
    let prefix = String::from("clicked");
    let button = Button {
        label: String::from("OK"),
        on_click: |label| println!("{prefix}: {label}"),
    };
    button.click();
    button.click();

    let mut seen = 0;
    let mut events = Events {
        handlers: Vec::new(),
    };
    events.handlers.push(Box::new(|e| e.len()));
    events.handlers.push(Box::new(move |_| {
        seen += 1;
        seen
    }));
    println!("fire 1: {:?}", events.fire("start"));
    println!("fire 2: {:?}", events.fire("stop"));
    println!("outer seen (the closure counts its own copy): {seen}");
}
