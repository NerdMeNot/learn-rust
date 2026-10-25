// The rules of std's vec!, without the `[x; n]` form
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
    let empty: Vec<i32> = my_vec![];
    let nums = my_vec![1, 2 + 3, 4,];
    let words = my_vec!["a", "b"];
    println!("empty: {empty:?}");
    println!("nums: {nums:?}");
    println!("words: {words:?}");
}
