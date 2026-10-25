fn main() {
    let mut nickname: Option<String> = Some(String::from("bo"));

    // as_ref: Option<&String>, so the String stays where it is.
    let length = nickname.as_ref().map(|n| n.len());
    println!("length: {length:?}");
    println!("nickname still there: {nickname:?}");

    // as_mut: Option<&mut String>, to change it in place.
    if let Some(n) = nickname.as_mut() {
        n.push_str("bby");
    }
    println!("after as_mut: {nickname:?}");

    // as_deref: Option<String> becomes Option<&str>, handy for a fallback.
    let shown: &str = nickname.as_deref().unwrap_or("anonymous");
    let none: Option<String> = None;
    let shown_none: &str = none.as_deref().unwrap_or("anonymous");
    println!("as_deref: {shown} and {shown_none}");

    // get and first return Option<&T>. copied and cloned make an owned value.
    let scores = [10, 20, 30];
    println!("get(1): {:?}", scores.get(1));
    println!("get(1).copied(): {:?}", scores.get(1).copied());
    println!("get(7).copied(): {:?}", scores.get(7).copied());
    let names = [String::from("ana"), String::from("bo")];
    let first: Option<String> = names.first().cloned();
    println!("first().cloned(): {first:?}");
}
