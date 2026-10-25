use std::fmt;
use std::mem::size_of;

struct UserId(u32);
struct OrderId(u32);

fn load_user(id: &UserId) -> String {
    format!("user #{}", id.0)
}

/// A wrapper around a foreign type, so that this crate may implement a foreign trait on it.
struct Names(Vec<String>);

impl fmt::Display for Names {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

fn main() {
    let user = UserId(7);
    let order = OrderId(7);
    println!("{}", load_user(&user));
    println!("order #{}", order.0);

    println!("size_of::<u32>():     {}", size_of::<u32>());
    println!("size_of::<UserId>():  {}", size_of::<UserId>());
    println!("size_of::<Names>():   {}", size_of::<Names>());
    println!("size_of::<Vec<String>>(): {}", size_of::<Vec<String>>());

    println!("{}", Names(vec!["Ada".to_string(), "Grace".to_string()]));
}
