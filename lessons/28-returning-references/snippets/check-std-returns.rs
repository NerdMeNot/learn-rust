fn main() {
    let s = String::from(" a=b ");
    let _: String = s.to_uppercase();
    let _: &str = s.trim();
    let _: Option<(&str, &str)> = s.split_once('=');
    let v = vec![1, 2];
    let _: Option<&i32> = v.get(0);
    let _: Vec<i32> = v[..1].to_vec();
    let mut out = String::new();
    out.push_str("x");
    let _: Option<&mut i32> = vec![1].get_mut(0);
}
