struct Button<F: Fn(&str)> {
    on_click: F,
}

fn main() {
    let button = Button {
        on_click: |label| println!("clicked {label}"),
    };
    button.on_click("OK");
}
