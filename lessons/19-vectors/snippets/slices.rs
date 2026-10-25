fn main() {
    let v = vec![10, 20, 30, 40, 50];

    println!("&v[1..3]: {:?}", &v[1..3]);
    println!("&v[..2]:  {:?}", &v[..2]);
    println!("&v[3..]:  {:?}", &v[3..]);
    println!("&v[..]:   {:?}", &v[..]);
    println!("get(3..9): {:?}", v.get(3..9));

    let (left, right) = v.split_at(2);
    println!("split_at(2): {left:?} and {right:?}");

    for pair in v.windows(2) {
        println!("window: {pair:?}");
    }
    for chunk in v.chunks(2) {
        println!("chunk:  {chunk:?}");
    }

    let rows = vec![vec![1, 2], vec![3], vec![4, 5]];
    println!("concat: {:?}", rows.concat());
    let words = vec!["a", "b", "c"];
    println!("join:   {}", words.join("-"));

    let copy: Vec<i32> = v[1..3].to_vec();
    println!("to_vec: {copy:?}");
}
