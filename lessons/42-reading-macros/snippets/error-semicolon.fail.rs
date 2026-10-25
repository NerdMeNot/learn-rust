macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($($x:expr),+ $(,)?) => {{
        let mut v = Vec::new();
        $(v.push($x);)+
        v
    }};
}

fn main() {
    let v = my_vec![0; 3];
    println!("{v:?}");
}
