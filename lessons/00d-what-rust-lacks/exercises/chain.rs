// user?.address?.city?.length ?? 0, in other languages, becomes a function that
// returns an Option and uses `?`: each `?` returns None from the function when its
// value is missing. An empty city is still a city, so its length is Some(0).
struct Address {
    city: Option<String>,
}

struct User {
    address: Option<Address>,
}

fn city_len(user: Option<&User>) -> Option<usize> {
    todo!("user?.address.as_ref()?.city.as_ref().map(...)")
}

fn main() {
    let home = User {
        address: Some(Address {
            city: Some("Paris".to_string()),
        }),
    };
    let nowhere = User { address: None };
    let unnamed = User {
        address: Some(Address { city: None }),
    };
    let empty = User {
        address: Some(Address {
            city: Some(String::new()),
        }),
    };
    assert_eq!(city_len(Some(&home)), Some(5));
    assert_eq!(city_len(None), None);
    assert_eq!(city_len(Some(&nowhere)), None);
    assert_eq!(city_len(Some(&unnamed)), None);
    assert_eq!(city_len(Some(&empty)), Some(0));
    println!("all checks passed");
}
