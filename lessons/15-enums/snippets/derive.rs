#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
enum Priority {
    Low,
    #[default]
    Normal,
    Urgent,
}

#[derive(Debug, Clone, PartialEq)]
enum Reply {
    Ok,
    Text(String),
    Number(f64),
}

fn main() {
    let p = Priority::default();
    let q = p; // Copy: p is still usable
    println!("default: {p:?}, copy: {q:?}");
    println!("Low < Urgent: {}", Priority::Low < Priority::Urgent);
    println!("max: {:?}", Priority::Low.max(Priority::Urgent));

    let r = Reply::Text(String::from("hi"));
    println!("clone equal: {}", r.clone() == r);
    println!(
        "same variant, other data: {}",
        r == Reply::Text(String::from("ho"))
    );
    println!("other variant: {}", r == Reply::Ok);
    println!("{:?} / {:?}", Reply::Number(1.5), Reply::Ok);
    println!("{:#?}", Reply::Text(String::from("pretty")));
}
