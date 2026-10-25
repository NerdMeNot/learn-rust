enum Input {
    Text(String),
    Number(f64),
}

fn show(x: Input) {
    match x {
        Input::Text(s) => println!("{}", s.to_uppercase()),
        Input::Number(n) => println!("{n:.2}"),
    }
}

fn main() {
    show(Input::Text("ana".to_string()));
    show(Input::Number(2.0));
}
